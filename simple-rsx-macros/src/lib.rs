use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    Expr, ExprLit, Ident, Lit, Result, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

/// A procedural macro that provides JSX-like syntax for creating HTML elements in Rust.
///
/// # Examples
///
/// ```rust
/// // Fragment
/// jsx!(<>Hello World</>);
///
/// // Self-closing tag
/// jsx!(<div class="container" id="app" />);
///
/// // Tag with children
/// jsx!(<div class="container">
///     <h1>Title</h1>
///     <p>Paragraph text</p>
/// </div>);
///
/// // Expression
/// let name = "World";
/// jsx!(<div>Hello {name}</div>);
/// ```
#[proc_macro]
pub fn jsx(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as JsxNode);
    let expanded = input.to_tokens();
    expanded.into()
}

/// Represents the different types of JSX nodes
enum JsxNode {
    Fragment(Vec<JsxNode>),
    Element {
        tag: Ident,
        attributes: Vec<(Ident, Lit)>,
        children: Vec<JsxNode>,
        close_tag: Option<Ident>, // Optional closing tag for elements
    },
    Text(Expr),
    Empty,
}

/// Represents an attribute name-value pair
struct Attribute {
    name: Ident,
    value: Lit,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![=]>()?;
        let value = input.parse()?;
        Ok(Attribute { name, value })
    }
}

impl Parse for JsxNode {
    fn parse(input: ParseStream) -> Result<Self> {
        // Empty
        if input.is_empty() {
            return Ok(JsxNode::Empty);
        }

        // Look ahead to see if we start with a '<'
        if input.peek(Token![<]) {
            input.parse::<Token![<]>()?;

            // Fragment: <>...</>
            if input.peek(Token![>]) {
                input.parse::<Token![>]>()?;

                // Parse children until we find </> closing tag
                let mut children = Vec::new();
                while !input.is_empty()
                    && !(input.peek(Token![<]) && input.peek2(Token![/]) && input.peek3(Token![>]))
                {
                    if let Ok(child) = input.parse::<JsxNode>() {
                        children.push(child);
                    } else {
                        // Skip a token if we can't parse a node
                        let _ = input.parse::<proc_macro2::TokenTree>();
                    }
                }

                // Consume </> closing tag
                input.parse::<Token![<]>()?;
                input.parse::<Token![/]>()?;
                input.parse::<Token![>]>()?;

                return Ok(JsxNode::Fragment(children));
            }

            // Element: <tag ...>...</tag> or <tag ... />
            let tag = input.parse::<Ident>()?;

            // Parse attributes
            let mut attributes = Vec::new();
            while !input.peek(Token![>]) && !input.peek(Token![/]) {
                let attr: Attribute = input.parse()?;
                attributes.push((attr.name, attr.value));
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

            // Parse children
            let mut children = Vec::new();
            while !input.is_empty() && !(input.peek(Token![<]) && input.peek2(Token![/])) {
                let child = input.parse::<JsxNode>()?;
                children.push(child);
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

        // Handle expressions in braces: {expr}
        // if input.peek(Token!['{']) {
        //     let content;
        //     syn::braced!(content in input);
        //     let expr: Expr = content.parse()?;
        //     return Ok(JsxNode::Text(expr));
        // }

        // Try to parse as an expression
        match input.parse::<Expr>() {
            Ok(expr) => Ok(JsxNode::Text(expr)),
            Err(_) => {
                // If we reach here, likely we have multiple sibling nodes
                // Since we don't have a way to directly detect this, we'll treat
                // unrecognized patterns as an error
                Err(syn::Error::new(
                    Span::call_site(),
                    "Expected a JSX element, fragment, text, or expression",
                ))
            }
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
                        let mut nodes = Vec::new();
                        #(
                            let result = #children_tokens;
                            match result {
                                simple_rsx::NodeList::Fragment(mut child_nodes) => nodes.append(&mut child_nodes),
                                simple_rsx::NodeList::Single(node) => nodes.push(node),
                            }
                        )*
                        simple_rsx::NodeList::Fragment(nodes)
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
                    let name_str = name.to_string();
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
                                simple_rsx::NodeList::Fragment(nodes) => {
                                    for child in nodes {
                                        #tag.append_child(child);
                                    }
                                },
                                simple_rsx::NodeList::Single(node) => {
                                    #tag.append_child(node);
                                }
                            }
                        )*
                    }
                };

                let close_tag = if let Some(close_tag) = close_tag {
                    quote! {
                        #close_tag = #tag;
                    }
                } else {
                    quote! {}
                };

                quote! {
                    {
                        #[allow(unused_mut)]
                        let mut #tag = simple_rsx::Element::new(#tag_str);
                        #(#attr_setters)*
                        #children_handlers
                        #close_tag
                        simple_rsx::NodeList::Single(#tag)
                    }
                }
            }
            JsxNode::Text(expr) => {
                quote! {
                    simple_rsx::NodeList::Single(simple_rsx::TextNode::new(&(#expr).to_string()))
                }
            }
            JsxNode::Empty => {
                quote! {
                    simple_rsx::NodeList::Fragment(Vec::new())
                }
            }
        }
    }
}
