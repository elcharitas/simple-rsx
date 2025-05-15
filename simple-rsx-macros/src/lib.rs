use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::spanned::Spanned;
use syn::{
    Block, Expr, ExprLit, Ident, Lit, LitStr, Macro, Result, Token,
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    token::{Brace, Not},
};

/// A procedural macro that provides JSX-like syntax for creating HTML elements in Rust.
///
/// # Examples
///
/// ```rust
/// use simple_rsx::*;
/// // Fragment
/// rsx!(<>Hello World</>);
///
/// // Self-closing tag
/// rsx!(<div class="container" id="app" />);
///
/// // Tag with children
/// rsx!(<div class="container">
///     <h1>Title</h1>
///     <p>Paragraph text</p>
/// </div>);
///
/// // Expression
/// let name = "World";
/// rsx!(<div>Hello {name}</div>);
/// ```
#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as JsxNode);
    let expanded = input.to_tokens();
    expanded.into()
}

/// Represents the different types of JSX nodes
enum JsxNode {
    Fragment(Vec<JsxNode>),
    Component {
        name: Ident,
        props: Vec<(Ident, Option<Block>)>,
        children: Vec<JsxNode>,
        close_tag: Option<Ident>,
    },
    Text(Expr),
    Block(Block),
    Empty,
    Comment(String), // HTML comments
}

struct NodeBlock {
    value: Option<Block>,
}

impl Parse for NodeBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitStr) {
            let parsed: LitStr = input.parse()?;
            return Ok(NodeBlock {
                value: Some(Block {
                    brace_token: Brace::default(),
                    stmts: vec![syn::Stmt::Expr(
                        syn::Expr::Macro(syn::ExprMacro {
                            attrs: Vec::new(),
                            mac: Macro {
                                path: parse_quote!(format),
                                bang_token: Not::default(),
                                delimiter: syn::MacroDelimiter::Paren(syn::token::Paren::default()),
                                tokens: quote::quote!(#parsed),
                            },
                        }),
                        None,
                    )],
                }),
            });
        }

        let is_block = input.to_string().trim().starts_with('{');

        if is_block {
            let value: Block = input.parse()?;
            return Ok(NodeBlock { value: Some(value) });
        }

        let mut str = String::new();
        let mut in_string = false;
        let mut last_end = 0;

        while !input.is_empty() {
            if input.lookahead1().peek(Token![<]) && !in_string {
                // Found a non-literal '<', stop here without consuming it
                break;
            }

            match input.parse::<proc_macro2::TokenTree>() {
                Ok(token) => {
                    match &token {
                        proc_macro2::TokenTree::Literal(lit) => {
                            let lit_str = lit.to_string();
                            in_string = lit_str.starts_with('"') || lit_str.starts_with('\'');
                        }
                        _ => in_string = false,
                    }

                    let span_info = format!("{:?}", token.span());
                    let (start, end) = parse_range(&span_info).unwrap_or((0, 0));

                    let mut value = token.to_string();

                    if value.starts_with('{') && value.ends_with('}') {
                        value = value.replace("{ ", "{");
                        value = value.replace(" }", "}");
                    }

                    if start > last_end {
                        str.push(' ');
                        last_end = end;
                    }
                    str.push_str(&value);
                }
                Err(_) => break, // End of input
            }
        }

        let lit = LitStr::new(&str.trim(), Span::call_site());

        Ok(NodeBlock {
            value: Some(Block {
                brace_token: Brace::default(),
                stmts: vec![syn::Stmt::Expr(
                    syn::Expr::Macro(syn::ExprMacro {
                        attrs: Vec::new(),
                        mac: Macro {
                            path: parse_quote!(format),
                            bang_token: Not::default(),
                            delimiter: syn::MacroDelimiter::Paren(syn::token::Paren::default()),
                            tokens: quote::quote!(#lit),
                        },
                    }),
                    None,
                )],
            }),
        })
    }
}

/// Represents an attribute name-value pair
struct NodeValue {
    name: Ident,
    value: Option<Block>,
}

impl Parse for NodeValue {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        if !input.peek(Token![=]) {
            return Ok(NodeValue { name, value: None });
        }
        input.parse::<Token![=]>()?;
        let NodeBlock { value } = input.parse()?;
        Ok(NodeValue { name, value })
    }
}

impl Parse for JsxNode {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Ok(JsxNode::Empty);
        }

        // Look ahead to see if we start with a '<'
        if input.peek(Token![<]) {
            input.parse::<Token![<]>()?;

            // Comments: <!-- ... -->
            if input.peek(Token![!]) && input.peek2(Token![-]) && input.peek3(Token![-]) {
                input.parse::<Token![!]>()?;
                input.parse::<Token![-]>()?;
                input.parse::<Token![-]>()?;

                let mut comment = String::new();
                let mut last_end = 0;
                while !input.is_empty()
                    && !(input.peek(Token![-]) && input.peek2(Token![-]) && input.peek3(Token![>]))
                {
                    let token = input.parse::<proc_macro2::TokenTree>()?;
                    let span_info = format!("{:?}", token.span());
                    let (start, end) = parse_range(&span_info).unwrap_or((0, 0));
                    if start > last_end {
                        comment.push(' ');
                        last_end = end;
                    }
                    comment.push_str(&token.to_string());
                }

                let token = input.parse::<Token![-]>()?;
                let span_info = format!("{:?}", token.span());
                let (start, _) = parse_range(&span_info).unwrap_or((0, 0));
                if start > last_end {
                    comment.push(' ');
                }
                input.parse::<Token![-]>()?;
                input.parse::<Token![>]>()?;

                return Ok(JsxNode::Comment(comment.to_string()));
            }

            // Fragment: <>...</>
            if input.peek(Token![>]) {
                input.parse::<Token![>]>()?;

                let mut children = Vec::with_capacity(4); // Pre-allocate with reasonable capacity
                while !input.is_empty()
                    && !(input.peek(Token![<]) && input.peek2(Token![/]) && input.peek3(Token![>]))
                {
                    match input.parse::<JsxNode>() {
                        Ok(child) => children.push(child),
                        Err(_) => {
                            input.parse::<proc_macro2::TokenTree>()?;
                        }
                    }
                }

                input.parse::<Token![<]>()?;
                input.parse::<Token![/]>()?;
                input.parse::<Token![>]>()?;

                return Ok(JsxNode::Fragment(children));
            }

            // Element: <tag ...>...</tag> or <tag ... />
            let tag = input.parse::<Ident>()?;

            let mut attributes = Vec::with_capacity(4);
            while !input.peek(Token![>]) && !input.peek(Token![/]) {
                match input.parse::<NodeValue>() {
                    Ok(attr) => attributes.push((attr.name, attr.value)),
                    Err(e) => return Err(e),
                }
            }

            // Self-closing tag: <tag ... /> or <Component... />
            if input.peek(Token![/]) {
                input.parse::<Token![/]>()?;
                input.parse::<Token![>]>()?;

                return Ok(JsxNode::Component {
                    name: tag.clone(),
                    props: attributes,
                    children: Vec::new(),
                    close_tag: None,
                });
            }

            // Opening tag ends: <tag ...>
            input.parse::<Token![>]>()?;

            let mut children = Vec::with_capacity(4);
            while !input.is_empty() && !(input.peek(Token![<]) && input.peek2(Token![/])) {
                match input.parse::<JsxNode>() {
                    Ok(child) => children.push(child),
                    Err(e) => return Err(e),
                }
            }

            // Closing tag: </tag>
            input.parse::<Token![<]>()?;
            input.parse::<Token![/]>()?;
            let close_tag = input.parse::<Ident>()?;

            // Validate matching tags
            if tag != close_tag {
                return Err(syn::Error::new(
                    close_tag.span(),
                    format!(
                        "Closing tag </{}> doesn't match opening tag <{}>",
                        close_tag, tag
                    ),
                ));
            }

            input.parse::<Token![>]>()?;

            return Ok(JsxNode::Component {
                name: tag,
                props: attributes,
                children,
                close_tag: Some(close_tag),
            });
        }

        // Text content or expression
        if input.peek(Lit) {
            let lit: Lit = input.parse()?;
            let expr = Expr::Lit(ExprLit {
                attrs: Vec::new(),
                lit,
            });
            return Ok(JsxNode::Text(expr));
        }
        match input.parse::<Block>() {
            Ok(block) => Ok(JsxNode::Block(block)),
            Err(_) => match input.parse::<NodeBlock>() {
                Ok(block) => match block.value {
                    Some(value) => Ok(JsxNode::Block(value)),
                    _ => Ok(JsxNode::Empty),
                },
                Err(_) => match input.parse::<Expr>() {
                    Ok(expr) => Ok(JsxNode::Text(expr)),
                    Err(_) => Err(syn::Error::new(
                        Span::call_site(),
                        "Invalid JSX node, expected a valid rsx block, an expression or plain text",
                    )),
                },
            },
        }
    }
}

impl JsxNode {
    fn to_tokens(&self) -> TokenStream2 {
        match self {
            JsxNode::Component {
                name,
                props,
                children,
                close_tag,
            } => {
                let props_tokens = props.iter().map(|(name, value)| {
                    if value.is_none() {
                        quote! {
                            #name: true,
                        }
                    } else {
                        quote! {
                            #name: #value.into(),
                        }
                    }
                });

                let children_tokens = if !children.is_empty() {
                    let child_tokens = children.iter().map(|child| child.to_tokens());
                    Some(quote! {
                        children: vec![#(#child_tokens),*],
                    })
                } else {
                    None
                };

                let close_tag = close_tag.as_ref().and_then(|close_tag| {
                    Some(quote! {
                        let #close_tag = #name;
                    })
                });

                quote! {
                    {
                        type Props = <#name as simple_rsx::Component>::Props;
                        #close_tag
                        simple_rsx::Component::render(
                            &mut #name,
                            Props {
                                #(#props_tokens)*
                                #children_tokens
                                ..Default::default()
                            },
                        )
                    }
                }
            }
            JsxNode::Fragment(children) => {
                let children_tokens = children.iter().map(|child| child.to_tokens());

                quote! {
                    {
                        simple_rsx::Node::Fragment(vec![#(#children_tokens)*])
                    }
                }
            }
            JsxNode::Text(expr) => {
                quote! {
                    simple_rsx::Node::Text(#expr.to_string())
                }
            }
            JsxNode::Empty => {
                quote! {
                    simple_rsx::Node::Fragment(Vec::new())
                }
            }
            JsxNode::Comment(text) => {
                quote! {
                    simple_rsx::Node::Comment(#text.to_string())
                }
            }
            JsxNode::Block(block) => {
                quote! {
                    simple_rsx::Node::from(#block)
                }
            }
        }
    }
}

fn parse_range(input: &str) -> Option<(usize, usize)> {
    use regex::Regex;
    let re = Regex::new(r"(\d+)\.\.(\d+)").ok()?;
    let captures = re.captures(input)?;
    let start = captures.get(1)?.as_str().parse::<usize>().ok()?;
    let end = captures.get(2)?.as_str().parse::<usize>().ok()?;

    Some((start, end))
}
