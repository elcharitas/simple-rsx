use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
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
    Element {
        tag: Ident,
        attributes: Vec<(Ident, Block)>,
        children: Vec<JsxNode>,
        close_tag: Option<Ident>, // Optional closing tag for elements
    },
    Text(Expr),
    Block(Block),
    Empty,
}

struct NodeBlock {
    value: Block,
}

impl Parse for NodeBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitStr) {
            let parsed: LitStr = input.parse()?;
            return Ok(NodeBlock {
                value: Block {
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
                },
            });
        }

        let is_block = input.to_string().trim().starts_with('{');

        if is_block {
            let value = input.parse()?;
            return Ok(NodeBlock { value });
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
            value: Block {
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
            },
        })
    }
}

/// Represents an attribute name-value pair
struct NodeValue {
    name: Ident,
    value: Block,
}

impl Parse for NodeValue {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
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

            // Self-closing tag: <tag ... />
            if input.peek(Token![/]) {
                input.parse::<Token![/]>()?;
                input.parse::<Token![>]>()?;

                return Ok(JsxNode::Element {
                    tag,
                    attributes,
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

            return Ok(JsxNode::Element {
                tag,
                attributes,
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
                Ok(block) => Ok(JsxNode::Block(block.value)),
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
            JsxNode::Fragment(children) => {
                let children_tokens = children.iter().map(|child| child.to_tokens());

                quote! {
                    {
                        simple_rsx::Node::Fragment(vec![#(#children_tokens)*])
                    }
                }
            }
            JsxNode::Element {
                tag,
                attributes,
                children,
                close_tag,
            } => {
                let tag_str = tag.to_string();
                let attr_setters = attributes.iter().map(|(name, value)| {
                    let name_str = name.to_string().replace("r#", "");
                    quote! {
                        if let Some(e) = #tag.as_element_mut() {
                            let #name = #value;
                            e.set_attribute(#name_str, #name);
                        }
                    }
                });

                let children_handlers = if children.is_empty() {
                    quote! {}
                } else {
                    let children_tokens = children.iter().map(|child| child.to_tokens());

                    quote! {
                        #(
                            let child_result = #children_tokens;
                            match child_result {
                                simple_rsx::Node::Fragment(nodes) => {
                                    for child in nodes {
                                        #tag.append_child(child);
                                    }
                                },
                                _ => {
                                    #tag.append_child(child_result);
                                }
                            }
                        )*
                    }
                };

                let close_tag = close_tag.as_ref().and_then(|close_tag| {
                    Some(quote! {
                        #close_tag = #tag;
                    })
                });

                quote! {
                    {
                        #[allow(unused_mut)]
                        let mut #tag = simple_rsx::Element::new(#tag_str);
                        #(#attr_setters)*
                        #children_handlers
                        #close_tag
                        #tag
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
