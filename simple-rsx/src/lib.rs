//! Simple RSX - A React-inspired JSX Library for Rust
//!
//! I created Simple RSX to bring the familiar feel of React's JSX to Rust projects. If you're coming
//! from a React background, you'll feel right at home. And if you're new to both, don't worry - I've made
//! it super intuitive while keeping all the type safety and performance benefits of Rust.
//!
//! # Why Simple RSX?
//!
//! I started this project while attempting to transit my [portfolio](https://elcharitas.wtf) from Next.js to Rust.
//! I wanted to keep my codebase as simple as possible, and I wanted to use Rust's powerful type system
//! to ensure that my components were always correct. I tried existing libraries like `yew` and `sycamore`,
//! but they were either too complex or didn't feel quite like React. And so, here we are.
//!
//! I know what you're thinking - "Another UI library?" But here's what makes Simple RSX special:
//!
//! - **React-like Syntax**: Write your templates using the `rsx!` macro - it's just like JSX!
//! - **Type Safety**: Get compile-time checks for your components and props
//! - **Zero Runtime Overhead**: All the magic happens at compile time
//! - **Familiar Patterns**: Components, props, fragments - all the React concepts you love
//!
//! # Let's Get Started!
//!
//! Here's a quick taste of what you can do:
//!
//! ```rust
//! use simple_rsx::*;
//!
//! // Create your first component - looks familiar, right?
//! let greeting = rsx!(
//!     <div class="greeting">
//!         <h1>Hello, {"World"}!</h1>
//!         <p>Welcome to Simple RSX</p>
//!     </div>
//! );
//!
//! // Turn it into HTML - perfect for server-side rendering (P.S: This to me is my favorite feature)
//! println!("{}", greeting);
//! ```
//!
//! # Features You'll Love
//!
//! ## JSX-style Elements - Write HTML, Get Rust
//!
//! ```rust
//! use simple_rsx::*;
//!
//! // Self-closing tags? Check!
//! let img = rsx!(<img src="image.jpg" alt="An image" />);
//!
//! // Nested elements? Of course!
//! let card = rsx!(
//!     <div class="card">
//!         <h2>Title</h2>
//!         <p>Content</p>
//!     </div>
//! );
//!
//! // Fragments? No problem! Just use <> and the children will be flattened
//! let fragment = rsx!(
//!     <>
//!         <h1>Title</h1>
//!         <p>No wrapper needed</p>
//!     </>
//! );
//! ```
//!
//! ## Dynamic Content - Make It Come Alive
//!
//! ```rust
//! use simple_rsx::*;
//!
//! let name = "World";
//! let count = 42;
//!
//! // Drop in any Rust expression with {}
//! let dynamic = rsx!(
//!     <div>
//!         <h1>Hello, {name}!</h1>
//!         <p>Count: {count}</p>
//!     </div>
//! );
//!
//! // Conditional rendering? Use the either! macro
//! let show = true;
//! let conditional = either!(show =>
//!     <p>Now you see me</p>
//! else
//!     <p>Now you don&apos;t</p>
//! );
//!
//! // Conditional classes? Easy!
//! let is_active = true;
//! let button = rsx!(
//!     <button class={if is_active { "active" } else { "" }}>
//!         Toggle
//!     </button>
//! );
//!
//! // Render lists with iterator magic
//! let items = vec!["A", "B", "C"];
//! let list = rsx!(
//!     <ul>
//!         {items.iter().map(|item| rsx!(<li>{item}</li>))}
//!     </ul>
//! );
//! ```
//!
//! ## Components and Props - Build Reusable UI
//!
//! ```rust
//! use simple_rsx::*;
//!
//! // Define your props - just like React's PropTypes
//! #[derive(Default)]
//! struct ButtonProps {
//!     text: String,
//!     variant: String,
//!     children: Vec<Node>,
//! }
//!
//! // Create a component - clean and simple
//! #[component]
//! fn Button(props: ButtonProps) -> Node {
//!     rsx!(
//!         <button class={format!("btn btn-{}", props.variant)}>
//!             {props.text}
//!             {props.children}
//!         </button>
//!     )
//! }
//!
//! // Use it anywhere!
//! let button = rsx!(
//!     <Button text="Click me" variant="primary">
//!         <span>"→"</span>
//!     </Button>
//! );
//! ```
//!
//! ## HTML Data attributes
//!
//! With simple RSX, HTML data attributes are the only props which do not get validated by the compiler.
//! This allows you to use any valid literal or expression in the value of a data attribute.
//!
//! ```rust
//! use simple_rsx::*;
//!
//! // Data attributes? No problem!
//! let element = rsx!(
//!     <div
//!         data_user="john"
//!         data_role="admin"
//!     />
//! );
//! ```
//!
pub mod dom;
pub mod signals;

use indexmap::IndexMap;
pub use simple_rsx_macros::{component, either, rsx};
use std::fmt::Display;

/// A trait for converting values into HTML attribute strings.
///
/// This trait is automatically implemented for any type that implements `ToString`,
/// making it easy to use various types as attribute values.
///
/// # Example
///
/// ```rust
/// use simple_rsx::*;
///
/// let element = rsx!(<div id="my-id" hidden={true} />);
/// ```
pub trait Attribute {
    fn value(&self) -> String;
}

/// A trait for handling optional attribute values.
///
/// This trait is automatically implemented for `Option<T>` where T implements `ToString`.
/// It allows for graceful handling of optional attributes, rendering them as empty strings when None.
///
/// # Example
///
/// ```rust
/// use simple_rsx::*;
///
/// let maybe_title = Some("Hello".to_string());
/// let element = rsx!(<div title={maybe_title} />);
/// ```
pub trait OptionAttribute {
    fn value(&self) -> String;
}

impl<T: ToString> Attribute for T {
    fn value(&self) -> String {
        self.to_string()
    }
}

impl<T: ToString> OptionAttribute for Option<T> {
    fn value(&self) -> String {
        match self {
            Some(t) => t.to_string(),
            None => String::new(),
        }
    }
}

#[derive(Debug)]
/// Represents an HTML element with its tag name, attributes, and children.
///
/// Elements are the building blocks of the RSX tree structure. Each element
/// can have attributes (like class, id, etc.) and can contain other elements
/// or text nodes as children.
///
/// You typically won't create Elements directly, but rather use the `rsx!` macro:
///
/// ```rust
/// use simple_rsx::*;
///
/// let element = rsx!(
///     <div class="container">
///         <p>Hello world!</p>
///     </div>
/// );
/// ```
pub struct Element {
    tag: String,
    attributes: IndexMap<String, String>,
    children: Vec<Node>,
}

impl Element {
    /// Creates a new Element node with the specified tag name.
    ///
    /// # Example
    ///
    /// ```rust
    /// use simple_rsx::*;
    ///
    /// let element = Element::parse_tag("div");
    /// assert!(matches!(element, Node::Element(_)));
    /// ```
    pub fn parse_tag(tag: &str) -> Node {
        Node::Element(Element {
            tag: tag.to_string(),
            attributes: IndexMap::new(),
            children: Vec::new(),
        })
    }

    /// Sets an attribute on the element.
    ///
    /// # Example
    ///
    /// ```rust
    /// use simple_rsx::*;
    ///
    /// let mut node = Element::parse_tag("div");
    /// let mut element = node.as_element_mut().unwrap();
    /// element.set_attribute("class", "container");
    /// ```
    pub fn set_attribute(&mut self, name: &str, value: impl Attribute) {
        self.attributes.insert(name.to_string(), value.value());
    }

    /// Adds a child node to this element.
    ///
    /// # Example
    ///
    /// ```rust
    /// use simple_rsx::*;
    ///
    /// let mut parent_node = Element::parse_tag("div");
    /// let mut parent = parent_node.as_element_mut().unwrap();
    /// parent.append_child(Element::parse_tag("p"));
    /// ```
    pub fn append_child(&mut self, node: Node) {
        self.children.push(node);
    }
}

impl Node {
    /// Attempts to get a mutable reference to the underlying Element if this node is an Element.
    ///
    /// Returns None if the node is not an Element (e.g., if it's Text or Fragment).
    pub fn as_element_mut(&mut self) -> Option<&mut Element> {
        match self {
            Node::Element(el) => Some(el),
            _ => None,
        }
    }

    /// Adds a child node if this node is an Element.
    ///
    /// This method has no effect if the node is not an Element.
    pub fn append_child(&mut self, node: Node) {
        if let Node::Element(el) = self {
            el.children.push(node);
        }
    }
}

/// A trait for creating reusable components.
///
/// Components are the heart of RSX's reusability model. They allow you to create
/// custom elements with their own logic and state.
///
/// # Example
///
/// ```rust
/// use simple_rsx::*;
///
/// struct Card;
/// #[derive(Default)]
/// struct CardProps {
///     title: String,
///     children: Vec<Node>,
/// }
///
/// impl Component for Card {
///     type Props = CardProps;
///     fn render(props: Self::Props) -> Node {
///         rsx!(
///             <div class="card">
///                 <h2>{props.title}</h2>
///                 <div class="card-content">{props.children}</div>
///             </div>
///         )
///     }
/// }
/// ```
pub trait Component {
    /// The type of props this component accepts
    type Props;

    /// Renders the component with the given props
    fn render(props: Self::Props) -> Node;
}

#[derive(Default)]
pub struct PropWithChildren {
    pub children: Vec<Node>,
}

#[derive(Debug)]
/// Represents a node in the RSX tree.
///
/// Nodes are the fundamental building blocks of RSX. They can be:
/// - Elements (like `<div>` or `<p>`)
/// - Text content
/// - Fragments (groups of nodes)
/// - Comments
///
/// # Example
///
/// ```rust
/// use simple_rsx::*;
///
/// let text_node = Node::Text("Hello".to_string());
/// let element_node = Element::parse_tag("div");
/// let fragment = Node::Fragment(vec![text_node, element_node]);
/// ```
pub enum Node {
    /// An HTML element with a tag name, attributes, and children
    Element(Element),
    /// Plain text content
    Text(String),
    /// A group of nodes without a wrapper element
    Fragment(Vec<Node>),
    /// An HTML comment
    Comment(String),
    Empty,
}

impl From<String> for Node {
    fn from(value: String) -> Self {
        Node::Text(value)
    }
}

impl From<&String> for Node {
    fn from(value: &String) -> Self {
        Node::Text(value.to_string())
    }
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        Node::Text(value.to_string())
    }
}

impl From<&&str> for Node {
    fn from(value: &&str) -> Self {
        Node::Text(value.to_string())
    }
}

impl<T> From<Vec<T>> for Node
where
    Node: From<T>,
{
    fn from(value: Vec<T>) -> Self {
        Node::Fragment(value.into_iter().map(|t| Node::from(t)).collect())
    }
}

impl<T: ToString> From<Option<T>> for Node {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(t) => Node::Text(t.to_string()),
            _ => Node::Text("".to_string()),
        }
    }
}

impl From<&Vec<String>> for Node {
    fn from(value: &Vec<String>) -> Self {
        Node::Fragment(
            value
                .iter()
                .map(|item| Node::Text(item.to_string()))
                .collect(),
        )
    }
}

impl From<i32> for Node {
    fn from(value: i32) -> Self {
        Node::Text(value.to_string())
    }
}

impl From<u32> for Node {
    fn from(value: u32) -> Self {
        Node::Text(value.to_string())
    }
}

impl From<u64> for Node {
    fn from(value: u64) -> Self {
        Node::Text(value.to_string())
    }
}

impl FromIterator<u32> for Node {
    fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> Self {
        let mut result = Vec::new();
        for item in iter {
            result.push(Node::Text(item.to_string()));
        }
        Node::Fragment(result)
    }
}

impl FromIterator<u64> for Node {
    fn from_iter<T: IntoIterator<Item = u64>>(iter: T) -> Self {
        let mut result = Vec::new();
        for item in iter {
            result.push(Node::Text(item.to_string()));
        }
        Node::Fragment(result)
    }
}

impl FromIterator<i32> for Node {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        let mut result = Vec::new();
        for item in iter {
            result.push(Node::Text(item.to_string()));
        }
        Node::Fragment(result)
    }
}

impl From<f32> for Node {
    fn from(value: f32) -> Self {
        Node::Text(value.to_string())
    }
}

impl From<bool> for Node {
    fn from(value: bool) -> Self {
        Node::Text(value.to_string())
    }
}

impl<I, F, R> From<std::iter::Map<I, F>> for Node
where
    I: Iterator,
    F: FnMut(I::Item) -> R,
    R: Into<Node>,
    Vec<Node>: FromIterator<R>,
{
    fn from(iter: std::iter::Map<I, F>) -> Self {
        let nodes: Vec<Node> = iter.collect();
        Node::from(nodes)
    }
}

#[cfg(feature = "wasm")]
pub struct EventCallback(Box<dyn FnMut(web_sys::EventTarget)>);

#[cfg(feature = "wasm")]
impl Default for EventCallback {
    fn default() -> Self {
        Self(Box::new(|_| {}))
    }
}

#[cfg(feature = "wasm")]
impl<T: Fn(web_sys::EventTarget) + 'static> From<T> for EventCallback {
    fn from(value: T) -> Self {
        Self(Box::new(value))
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Element(el) => {
                write!(f, "<{}", el.tag)?;
                for (key, value) in &el.attributes {
                    write!(f, " {}=\"{}\"", key, value)?;
                }
                write!(f, ">")?;
                for child in &el.children {
                    write!(f, "{}", child)?;
                }
                write!(f, "</{}>", el.tag)?;
                Ok(())
            }
            Node::Text(text) => {
                write!(f, "{}", sanitize_html(text))?;
                Ok(())
            }
            Node::Fragment(nodes) => {
                for node in nodes {
                    write!(f, "{}", node)?;
                }
                Ok(())
            }
            Node::Comment(comment) => {
                write!(f, "<!--{}-->", comment)?;
                Ok(())
            }
            Node::Empty => {
                write!(f, "")?;
                Ok(())
            }
        }
    }
}

fn sanitize_html(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        match c {
            '<' => {
                result.push_str("&lt;");
            }
            '>' => {
                result.push_str("&gt;");
            }
            '&' => {
                result.push_str("&amp;");
            }
            '"' => {
                result.push_str("&quot;");
            }
            '\'' => {
                result.push_str("&#39;");
            }
            '/' => {
                result.push_str("&#x2F;");
            }
            _ => {
                result.push(c);
            }
        };
    }
    result
}

macro_rules! derive_elements {
    (
        $(
            $(#[$tag_meta:meta])*
            $tag:ident {
                $(
                    $(#[$attr_meta:meta])*
                    $attr_name:ident : $attr_value:ty
                ),* $(,)?
            }
        )*
    ) => {
        $(
            #[allow(non_camel_case_types)]
            $(#[$tag_meta])*
            pub struct $tag;

            paste::paste! {
                #[derive(Default)]
                #[allow(non_snake_case)]
                pub struct [<HTML $tag:camel Element Props>] {
                    // Global HTML attributes

                    /// The id attribute specifies a unique id for an HTML element
                    pub id: String,

                    /// A unique key to identify the element
                    pub key: String,

                    /// The class attribute specifies one or more class names for an HTML element
                    pub class: String,

                    /// The child nodes of the element
                    pub children: Vec<Node>,

                    /// The style attribute specifies an inline CSS style for an element
                    pub style: String,

                    /// The title attribute specifies extra information about an element (displayed as a tooltip)
                    pub title: Option<String>,
                    /// The width attribute specifies the width of the image
                    pub width: Option<String>,
                    /// The height attribute specifies the height of the image
                    pub height: Option<String>,

                    /// Specifies whether an element is draggable or not
                    pub draggable: bool,

                    /// Specifies visibility of an element (hidden or visible)
                    pub hidden: bool,

                    /// Specifies a shortcut key to activate/focus an element
                    pub accesskey: String,

                    /// Specifies whether the content of an element is editable or not
                    pub contenteditable: bool,

                    /// Specifies the text direction for the content in an element
                    pub dir: String,

                    /// Specifies the tabbing order of an element (when the tab button is used)
                    pub tabindex: Option<i32>,

                    /// Specifies whether the element is to have its spelling and grammar checked
                    pub spellcheck: bool,

                    /// Specifies the language of the element's content
                    pub lang: String,

                    /// Specifies whether an element is translateable or not
                    pub translate: bool,

                    /// Controls whether and how text input is automatically capitalized
                    pub autocapitalize: String,

                    /// Specifies an inline CSS style for an element
                    pub role: String,

                    /// Custom data attributes (data-*)
                    pub r#data: std::collections::HashMap<String, String>,

                    // ARIA Accessibility attributes

                    /// Identifies the current element within a set
                    pub aria_current: String,

                    /// Defines a string value that labels the current element
                    pub aria_label: Option<String>,

                    /// Identifies the element that labels the current element
                    pub aria_labelledby: Option<String>,

                    /// Identifies the element that describes the current element
                    pub aria_describedby: Option<String>,

                    /// Indicates whether an element is expanded or collapsed
                    pub aria_expanded: bool,

                    /// Indicates the element that represents the current item within a container or set
                    pub aria_selected: bool,

                    /// Indicates whether the element is checked, unchecked, or represents mixed mode
                    pub aria_checked: String,

                    /// Indicates whether an element and its subtree are hidden
                    pub aria_hidden: bool,

                    /// Indicates the availability and type of interactive popup element
                    pub aria_haspopup: String,

                    /// Defines an element's role
                    pub aria_role: String,

                    #[cfg(feature = "wasm")]
                    pub on_click: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_keydown: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_keyup: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_keypress: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_focus: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_blur: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_change: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_input: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_submit: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_reset: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_mouseover: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_mouseout: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_mousedown: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_mouseup: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_mousemove: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_mouseenter: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_mouseleave: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_mousewheel: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_scroll: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_load: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_unload: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_abort: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_error: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_resize: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_cut: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_copy: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_paste: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_contextmenu: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_dblclick: EventCallback,
                    #[cfg(feature = "wasm")]
                    pub on_drop: EventCallback,

                    // Element specific attributes
                    $(
                        pub $attr_name: $attr_value,
                    )*
                }

                impl [<HTML $tag:camel Element Props>] {
                    fn to_attributes(&self) -> IndexMap<String, String> {
                        #[allow(unused_mut)]
                        let mut attributes = IndexMap::new();
                        $(
                            if !self.$attr_name.value().is_empty() {
                                let mut key = stringify!($attr_name);
                                if let Some(last_char) = key.chars().last() {
                                    if last_char == '_' {
                                        key = &key[..key.len() - 1];
                                    }
                                }
                                attributes.insert(key.replace('_', "-"), self.$attr_name.value());
                            }
                        )*
                        if !self.id.value().is_empty() {
                            attributes.insert("id".to_string(), self.id.value());
                        }
                        if !self.class.value().is_empty() {
                            attributes.insert("class".to_string(), self.class.value());
                        }
                        if !self.style.value().is_empty() {
                            attributes.insert("style".to_string(), self.style.value());
                        }
                        if !self.title.value().is_empty() {
                            attributes.insert("title".to_string(), self.title.value());
                        }
                        if self.draggable {
                            attributes.insert("draggable".to_string(), "true".to_string());
                        }
                        if self.hidden {
                            attributes.insert("hidden".to_string(), "true".to_string());
                        }
                        if !self.accesskey.value().is_empty() {
                            attributes.insert("accesskey".to_string(), self.accesskey.value());
                        }
                        if self.contenteditable {
                            attributes.insert("contenteditable".to_string(), "true".to_string());
                        }
                        if !self.dir.value().is_empty() {
                            attributes.insert("dir".to_string(), self.dir.value());
                        }
                        if let Some(tabindex) = self.tabindex {
                            attributes.insert("tabindex".to_string(), tabindex.to_string());
                        }
                        if self.spellcheck {
                            attributes.insert("spellcheck".to_string(), "true".to_string());
                        }
                        if !self.lang.value().is_empty() {
                            attributes.insert("lang".to_string(), self.lang.value());
                        }
                        if self.translate {
                            attributes.insert("translate".to_string(), "true".to_string());
                        }
                        if !self.autocapitalize.value().is_empty() {
                            attributes.insert("autocapitalize".to_string(), self.autocapitalize.value());
                        }
                        if !self.role.value().is_empty() {
                            attributes.insert("role".to_string(), self.role.value());
                        }
                        if !self.aria_current.value().is_empty() {
                            attributes.insert("aria-current".to_string(), self.aria_current.value());
                        }
                        if !self.aria_label.value().is_empty() {
                            attributes.insert("aria-label".to_string(), self.aria_label.value());
                        }
                        if !self.aria_labelledby.value().is_empty() {
                            attributes.insert("aria-labelledby".to_string(), self.aria_labelledby.value());
                        }
                        if !self.aria_describedby.value().is_empty() {
                            attributes.insert("aria-describedby".to_string(), self.aria_describedby.value());
                        }
                        if self.aria_expanded {
                            attributes.insert("aria-expanded".to_string(), "true".to_string());
                        }
                        if self.aria_selected {
                            attributes.insert("aria-selected".to_string(), "true".to_string());
                        }
                        if !self.aria_checked.value().is_empty() {
                            attributes.insert("aria-checked".to_string(), self.aria_checked.value());
                        }
                        if self.aria_hidden {
                            attributes.insert("aria-hidden".to_string(), "true".to_string());
                        }
                        if !self.aria_haspopup.value().is_empty() {
                            attributes.insert("aria-haspopup".to_string(), self.aria_haspopup.value());
                        }
                        if !self.aria_role.value().is_empty() {
                            attributes.insert("aria-role".to_string(), self.aria_role.value());
                        }
                        // Add data-* attributes
                        for (key, value) in &self.r#data {
                            if key.starts_with("data_") {
                                attributes.insert(key.replace("_", "-"), value.clone());
                            } else {
                                attributes.insert(format!("data-{}", key), value.clone());
                            }
                        }

                        attributes
                    }
                }

                impl Component for $tag {
                    type Props = [<HTML $tag:camel Element Props>];

                    fn render(props: Self::Props) -> Node {
                        Node::Element(Element {
                            tag: stringify!($tag).to_string(),
                            attributes: props.to_attributes(),
                            children: props.children,
                        })
                    }
                }
            }
        )*
    };
}

pub mod elements {
    use super::*;
    derive_elements! {
        /// HTML `<html>` element - Root element of an HTML document
        html {
        }
        /// HTML `<body>` element - Represents the content of an HTML document
        ///
        /// Example:
        ///
        /// ```<body>Content goes here</body>```
        body {
        }
        /// HTML `<head>` element - Contains metadata about the document
        ///
        /// Example:
        ///
        /// ```<head><title>Document Title</title></head>```
        head {
        }
        /// HTML `<title>` element - Defines the title of the document
        ///
        /// Example:
        ///
        /// ```<title>Document Title</title>```
        title {
        }
        /// HTML `<meta` element - Provides metadata about the document
        ///
        /// Example:
        ///
        /// ```<meta charset="UTF-8">```
        meta {
            /// The character encoding of the document
            charset: String,
            /// The HTTP response status code
            http_equiv: String,
            /// The content of the document
            content: String,
            /// The name of the metadata
            name: String,
            /// The property of the metadata
            property: String,
        }
        /// HTML `<style>` element - Defines style information for a document
        ///
        /// Example:
        ///
        /// ```<style>body { background-color: #f0f0f0; }</style>```
        style {
        }
        /// HTML `<script>` element - Embeds executable code or data
        ///
        /// Example:
        ///
        /// ```<script src="script.js"></script>```
        script {
            /// The URL of the external script file
            src: String,
            /// The type of the script
            type_: String,
            /// The language of the script
            language: String,
            /// The character encoding of the script
            charset: String,
            /// The defer attribute specifies that the script will be executed after the document is finished parsing
            defer: Option<bool>,
            /// The async attribute specifies that the script will be executed asynchronously
            async_: Option<bool>,
        }
        /// HTML `<link>` element - Specifies relationships between the current document and an external resource
        ///
        /// Example:
        ///
        /// ```<link rel="stylesheet" href="style.css">```
        link {
            /// The relationship between the current document and the linked resource
            rel: String,
            /// The URL of the linked resource
            href: String,
            /// The type of the linked resource
            type_: String,
            /// The character encoding of the linked resource
            charset: String,
            /// crossorigin attribute specifies that the element will request a cross-origin Isolation of its browsing context
            crossorigin: String,
            /// The referrerpolicy attribute specifies the referrer policy for the linked resource
            referrerpolicy: String,
        }
        /// HTML `<div>` element - Container element for grouping and styling content
        ///
        /// Example:
        ///
        /// ```<div class="container">Content goes here</div>```
        div {
        }

        /// HTML `<p>` element - Represents a paragraph of text
        ///
        /// Example:
        ///
        /// ```<p>This is a paragraph of text.</p>```
        p {
        }

        /// HTML `<span>` element - Inline container for targeting text with styles
        ///
        /// Example:
        ///
        /// ```<span class="highlight">Highlighted text</span>```
        span {
        }

        /// HTML `<a>` element - Creates a hyperlink to other web pages or resources
        ///
        /// Example:
        ///
        /// ```<a href="https://example.com" target="_blank">Visit Example</a>```
        a {
            /// The href attribute specifies the URL of the page the link goes to
            /// Example: href="https://example.com"
            href: String,
            /// The target attribute specifies where to open the linked document
            /// Example: target="_blank" (opens in new tab)
            target: String,
            /// The rel attribute specifies the relationship between the current document and the linked document
            /// Example: rel="nofollow" (tells search engines not to follow this link)
            rel: String,
            /// The download attribute indicates the browser to download the URL instead of navigating
            /// Example: download="filename.pdf"
            download: String,
            /// The hreflang attribute specifies the language of the linked document
            /// Example: hreflang="en" (English)
            hreflang: String,
            /// The type attribute specifies the media type of the linked document
            /// Example: type="text/html"
            type_: String,
            /// The media attribute specifies what media/device the linked document is optimized for
            /// Example: media="print" (for print stylesheets)
            media: String,
            /// The referrerpolicy attribute specifies which referrer information to send
            /// Example: referrerpolicy="no-referrer"
            referrerpolicy: String,
            /// The ping attribute specifies URLs to be notified when the link is followed
            /// Example: ping="https://example.com/track"
            ping: String,
        }

        /// HTML `<h1>` element - First level heading (most important)
        ///
        /// Example:
        ///
        /// ```<h1>Main Page Title</h1>```
        h1 {
        }

        /// HTML `<h2>` element - Second level heading
        ///
        /// Example:
        ///
        /// ```<h2>Section Heading</h2>```
        h2 {
        }

        /// HTML `<h3>` element - Third level heading
        ///
        /// Example:
        ///
        /// ```<h3>Subsection Heading</h3>```
        h3 {
        }

        /// HTML `<h4>` element - Fourth level heading
        ///
        /// Example:
        ///
        /// ```<h4>Sub-subsection Heading</h4>```
        h4 {
        }

        /// HTML `<h5>` element - Fifth level heading
        ///
        /// Example:
        ///
        /// ```<h5>Minor Heading</h5>```
        h5 {
        }

        /// HTML `<h6>` element - Sixth level heading (least important)
        ///
        /// Example:
        ///
        /// ```<h6>Fine Detail Heading</h6>```
        h6 {
        }

        /// HTML `<img>` element - Embeds an image into the document
        ///
        /// Example:
        ///
        /// ```<img src="image.jpg" alt="Description of image">```
        img {
            /// The src attribute specifies the URL/path to the image
            /// Example: src="images/logo.png"
            src: String,
            /// The alt attribute provides alternative text for screen readers and if image fails to load
            /// Example: alt="Company Logo"
            alt: String,
            /// The loading attribute indicates how the browser should load the image
            /// Example: loading="lazy" (defers loading until it's near viewport)
            loading: String,
        }

        /// HTML `<br>` element - Produces a line break in text
        ///
        /// Example:
        ///
        /// ```<br>```
        br {}

        /// HTML `<hr>` element - Creates a horizontal rule (divider)
        ///
        /// Example:
        ///
        /// ```<hr>```
        hr {
        }

        /// HTML `<ul>` element - Unordered list with bullet points
        ///
        /// Example:
        ///
        /// ```<ul><li>Item 1</li><li>Item 2</li></ul>```
        ul {
            /// The type attribute specifies the bullet style (disc, circle, square)
            /// Example: type="square"
            type_: String,
        }

        /// HTML `<li>` element - List item within ordered or unordered lists
        ///
        /// Example:
        ///
        /// ```<li>List item content</li>```
        li {
            /// The value attribute specifies the start value of the list item (for ol)
            /// Example: value="3" (starts this item at number 3)
            value: Option<i32>,
        }

        /// HTML `<ol>` element - Ordered (numbered) list
        ///
        /// Example:
        ///
        /// ```<ol start="5" type="A"><li>Item E</li><li>Item F</li></ol>```
        ol {
            /// The type attribute specifies the numbering type (1, A, a, I, i)
            /// Example: type="A" (uses capital letters)
            type_: String,
            /// The start attribute specifies the start value of the list
            /// Example: start="5" (starts counting from 5)
            start: i32,
            /// The reversed attribute specifies that list should be in descending order
            /// Example: reversed (counts down instead of up)
            reversed: Option<bool>,
        }

        /// HTML `<table>` element - Creates a data table with rows and columns
        ///
        /// Example:
        ///
        /// ```<table border="1"><tr><th>Header</th></tr><tr><td>Data</td></tr></table>```
        table {
            /// The border attribute specifies the width of the border around the table
            /// Example: border="1" (1 pixel border)
            border: i32,
            /// The cellpadding attribute specifies the space between cell content and borders
            /// Example: cellpadding="5" (5 pixels of padding)
            cellpadding: i32,
            /// The cellspacing attribute specifies the space between cells
            /// Example: cellspacing="2" (2 pixels between cells)
            cellspacing: i32,
        }

        /// HTML `<tr>` element - Table row container
        ///
        /// Example:
        ///
        /// ```<tr><td>Cell 1</td><td>Cell 2</td></tr>```
        tr {
        }

        /// HTML `<td>` element - Table data cell
        ///
        /// Example:
        ///
        /// ```<td colspan="2">This cell spans two columns</td>```
        td {
            /// The colspan attribute specifies how many columns a cell should span
            /// Example: colspan="3" (cell spans 3 columns)
            colspan: i32,
            /// The rowspan attribute specifies how many rows a cell should span
            /// Example: rowspan="2" (cell spans 2 rows)
            rowspan: i32,
            /// The headers attribute associates data cells with header cells
            /// Example: headers="col1 row1" (associates with those header IDs)
            headers: String,
            /// The scope attribute specifies whether header cells are for rows or columns
            /// Example: scope="col" (header applies to whole column)
            scope: String,
        }

        /// HTML `<th>` element - Table header cell
        ///
        /// Example:
        ///
        /// ```<th scope="col">Column Header</th>```
        th {
            /// The colspan attribute specifies how many columns a cell should span
            /// Example: colspan="3" (header spans 3 columns)
            colspan: i32,
            /// The rowspan attribute specifies how many rows a cell should span
            /// Example: rowspan="2" (header spans 2 rows)
            rowspan: i32,
            /// The headers attribute associates data cells with header cells
            /// Example: headers="col1 row1" (associates with those header IDs)
            headers: String,
            /// The scope attribute specifies whether the header cell is for a row, column, etc.
            /// Example: scope="row" (header applies to whole row)
            scope: String,
        }

        /// HTML `<tbody>` element - Groups body content in a table
        ///
        /// Example:
        ///
        /// ```<table><tbody><tr><td>Data</td></tr></tbody></table>```
        tbody {
        }

        /// HTML `<thead>` element - Groups header content in a table
        ///
        /// Example:
        ///
        /// ```<table><thead><tr><th>Header</th></tr></thead><tbody>...</tbody></table>```
        thead {
        }

        /// HTML `<tfoot>` element - Groups footer content in a table
        ///
        /// Example:
        ///
        /// ```<table><thead>...</thead><tbody>...</tbody><tfoot><tr><td>Summary</td></tr></tfoot></table>```
        tfoot {
        }

        /// HTML `<form>` element - Container for interactive inputs to collect user data
        ///
        /// Example:
        ///
        /// ```<form action="/submit" method="post"><input type="text"><button type="submit">Submit</button></form>```
        form {
            /// The action attribute specifies where to send form data when submitted
            ///
            /// Example: action="/process-form.php"
            action: String,
            /// The method attribute specifies HTTP method for sending data (GET/POST)
            ///
            /// Example: method="post" (sends data in request body)
            method: String,
            /// The target attribute specifies where to display the response
            ///
            /// Example: target="_blank" (opens response in new tab)
            target: String,
            /// The enctype attribute specifies how form data should be encoded
            ///
            /// Example: enctype="multipart/form-data" (needed for file uploads)
            enctype: String,
            /// The novalidate attribute disables browser's built-in form validation
            ///
            /// Example: novalidate (skips validation)
            novalidate: Option<bool>,
            /// The autocomplete attribute controls browser autofill behavior
            ///
            /// Example: autocomplete="off" (disables autofill)
            autocomplete: String,
            /// The accept attribute specifies file types the server accepts (for file inputs)
            ///
            /// Example: accept=".jpg,.png" (accepts only those image formats)
            accept: String,
            /// Example: name="contact-form"
            name: String,
        }

        /// HTML `<input>` element - Creates interactive controls for forms
        ///
        /// Example:
        ///
        /// ```<input type="text" placeholder="Enter your name" required>```
        input {
            /// The type attribute specifies the input type (text, password, email, etc.)
            ///
            /// Example: type="email" (validates as email address)
            type_: String,
            /// The placeholder attribute shows hint text when field is empty
            ///
            /// Example: placeholder="Enter your email"
            placeholder: Option<String>,
            /// The required attribute makes the field mandatory
            ///
            /// Example: required (field must be filled)
            required: Option<bool>,
            /// The value attribute specifies the default/current value
            ///
            /// Example: value="Default text"
            value: Option<String>,
            /// The name attribute specifies the name of the input (for form submission)
            ///
            /// Example: name="email"
            name: String,
            /// The disabled attribute disables the input
            ///
            /// Example: disabled (user cannot interact with input)
            disabled: Option<bool>,
            /// The readonly attribute makes the input read-only
            ///
            /// Example: readonly (user cannot modify but can focus/select)
            readonly: Option<bool>,
            /// The min attribute specifies minimum value for number/date inputs
            ///
            /// Example: min="1" (number input minimum value)
            min: Option<String>,
            /// The max attribute specifies maximum value for number/date inputs
            ///
            /// Example: max="100" (number input maximum value)
            max: Option<String>,
            /// The pattern attribute specifies a regex pattern for validation
            ///
            /// Example: pattern="[0-9]{3}" (requires exactly 3 digits)
            pattern: Option<String>,
            /// The autocomplete attribute controls browser autofill for this field
            ///
            /// Example: autocomplete="current-password"
            autocomplete: Option<String>,
        }

        /// HTML `<textarea>` element - Multi-line text input control
        ///
        /// Example:
        ///
        /// ```<textarea rows="4" cols="50" placeholder="Your message here"></textarea>```
        textarea {
            /// The placeholder attribute shows hint text when field is empty
            /// Example: placeholder="Enter your comments"
            placeholder: String,
            /// The required attribute makes the field mandatory
            /// Example: required (must be filled before submission)
            required: Option<bool>,
            /// The value attribute specifies the default/current text content
            /// Example: value="Default text in the textarea"
            value: String,
            /// The rows attribute specifies visible number of text lines
            /// Example: rows="10" (shows 10 lines of text)
            rows: Option<i32>,
            /// The cols attribute specifies visible width in average characters
            /// Example: cols="40" (about 40 characters wide)
            cols: Option<i32>,
            /// The name attribute specifies the name of the textarea (for form submission)
            /// Example: name="comments"
            name: String,
            /// The disabled attribute disables the textarea
            /// Example: disabled (user cannot interact)
            disabled: Option<bool>,
            /// The readonly attribute makes the textarea read-only
            /// Example: readonly (user cannot modify but can focus/select)
            readonly: Option<bool>,
            /// The maxlength attribute specifies maximum character count
            /// Example: maxlength="500" (limits to 500 characters)
            maxlength: Option<i32>,
        }

        /// HTML `<button>` element - Clickable button control
        ///
        /// Example:
        ///
        /// ```<button type="submit">Click Me</button>```
        button {
            /// The type attribute specifies button function (submit, reset, button)
            /// Example: type="submit" (submits the form)
            type_: String,
            /// The value attribute specifies the value associated with the button
            /// Example: value="btn1" (for form processing)
            value: String,
            /// The disabled attribute disables the button
            /// Example: disabled (button cannot be clicked)
            disabled: Option<bool>,
            /// The name attribute specifies the name of the button (for form submission)
            /// Example: name="submit-button"
            name: String,
            /// The formaction attribute overrides form's action for this button
            /// Example: formaction="/alternative-submit"
            formaction: String,
            /// The formmethod attribute overrides form's method for this button
            /// Example: formmethod="get"
            formmethod: String,
        }

        /// HTML `<select>` element - Dropdown selection list
        ///
        /// Example:
        ///
        /// ```<select><option value="1">Option 1</option><option value="2">Option 2</option></select>```
        select {
            /// The multiple attribute allows selecting multiple options
            /// Example: multiple (user can select multiple items)
            multiple: Option<bool>,
            /// The disabled attribute disables the dropdown
            /// Example: disabled (user cannot interact)
            disabled: Option<bool>,
            /// The value attribute specifies the selected value
            /// Example: value="option2" (preselects this option)
            value: String,
            /// The name attribute specifies the name of the select (for form submission)
            /// Example: name="country"
            name: String,
            /// The size attribute specifies number of visible options
            /// Example: size="5" (shows 5 options at once)
            size: i32,
            /// The required attribute makes selection mandatory
            /// Example: required (user must select an option)
            required: Option<bool>,
        }

        /// HTML `<option>` element - Defines option in a select dropdown
        ///
        /// Example:
        ///
        /// ```<option value="blue" selected>Blue</option>```
        option {
            /// The value attribute specifies the value to be sent to server
            /// Example: value="NY" (value sent when this option is selected)
            value: String,
            /// The selected attribute preselects this option when page loads
            /// Example: selected (this option is selected by default)
            selected: Option<bool>,
            /// The disabled attribute makes this option unselectable
            /// Example: disabled (cannot be chosen)
            disabled: Option<bool>,
        }

        /// HTML `<label>` element - Caption for a form control
        ///
        /// Example:
        ///
        /// ```<label for="username">Username:</label><input id="username">```
        label {
            /// The for attribute connects the label to a form control by ID
            /// Example: for="email" (associates with input having id="email")
            for_: String,
        }

        /// HTML `<iframe>` element - Embeds another document within the current HTML document
        ///
        /// Example:
        ///
        /// ```<iframe src="https://example.com" title="Example Site"></iframe>```
        iframe {
            /// The src attribute specifies the URL of the embedded document
            /// Example: src="https://maps.google.com"
            src: String,
            /// The frameborder attribute specifies whether to display a border
            /// Example: frameborder="0" (no border)
            frameborder: String,
            /// The allow attribute specifies features allowed in the iframe
            /// Example: allow="camera; microphone" (permits access to these devices)
            allow: String,
            /// The allowfullscreen attribute allows iframe content to go fullscreen
            /// Example: allowfullscreen (allows fullscreen mode)
            allowfullscreen: Option<bool>,
            /// The sandbox attribute restricts iframe capabilities for security
            /// Example: sandbox="allow-scripts" (only allows scripts to run)
            sandbox: String,
        }

        /// HTML `<video>` element - Embeds video content in the document
        ///
        /// Example:
        ///
        /// ```<video src="movie.mp4" controls width="500">Video not supported</video>```
        video {
            /// The src attribute specifies URL/path of the video
            /// Example: src="videos/intro.mp4"
            src: String,
            /// The controls attribute displays video playback controls
            /// Example: controls (shows play/pause/volume controls)
            controls: Option<bool>,
            /// The autoplay attribute starts playing video automatically
            /// Example: autoplay (video plays when page loads)
            autoplay: Option<bool>,
            /// The loop attribute makes the video replay when finished
            /// Example: loop (continuously replays)
            loop_: Option<bool>,
            /// The poster attribute specifies an image shown before video plays
            /// Example: poster="thumbnail.jpg"
            poster: String,
            /// The muted attribute mutes the audio by default
            /// Example: muted (starts with no sound)
            muted: Option<bool>,
            /// The preload attribute hints how to preload the video
            /// Example: preload="auto" (preload entire video)
            preload: String,
            /// The playsinline attribute plays inline on iOS (instead of fullscreen)
            /// Example: playsinline (important for iPhone users)
            playsinline: Option<bool>,
        }

        /// HTML `<audio>` element - Embeds sound content in the document
        ///
        /// Example:
        ///
        /// ```<audio src="song.mp3" controls>Audio not supported</audio>```
        audio {
            /// The src attribute specifies URL/path of the audio file
            /// Example: src="audio/background-music.mp3"
            src: String,
            /// The controls attribute displays audio playback controls
            /// Example: controls (shows play/pause/volume controls)
            controls: Option<bool>,
            /// The autoplay attribute starts playing audio automatically
            /// Example: autoplay (audio plays when page loads)
            autoplay: Option<bool>,
            /// The loop attribute makes the audio replay when finished
            /// Example: loop (continuously replays)
            loop_: Option<bool>,
            /// The muted attribute mutes the audio by default
            /// Example: muted (starts with no sound)
            muted: Option<bool>,
            /// The preload attribute hints how to preload the audio
            /// Example: preload="none" (doesn't preload)
            preload: String,
        }

        /// HTML `<source>` element - Defines media resources for video/audio elements
        ///
        /// Example:
        ///
        /// ```<video><source src="movie.mp4" type="video/mp4"><source src="movie.webm" type="video/webm"></video>```
        source {
            /// The src attribute specifies URL/path of the media resource
            /// Example: src="audio/song.ogg"
            src: String,
            /// The type attribute specifies the MIME type of the resource
            /// Example: type="video/webm" (defines file format)
            type_: String,
            /// The media attribute specifies for which media the resource is intended
            /// Example: media="(min-width: 600px)" (responsive resources)
            media: String,
        }

        /// HTML `<canvas>` element - Container for graphics rendered with JavaScript
        ///
        /// Example:
        ///
        /// ```<canvas id="myCanvas" width="200" height="100">Your browser does not support canvas</canvas>```
        canvas {
        }

        /// HTML `<svg>` element - Container for SVG graphics
        ///
        /// Example:
        ///
        /// ```<svg viewBox="0 0 100 100"><circle cx="50" cy="50" r="40" fill="red" /></svg>```
        svg {
            /// The viewBox attribute defines coordinate system and aspect ratio
            /// Example: viewBox="0 0 800 600" (x, y, width, height)
            viewBox: String,
            /// The preserveAspectRatio attribute controls scaling behavior
            /// Example: preserveAspectRatio="xMidYMid meet" (center and scale)
            preserve_aspect_ratio: String,
            /// The xmlns attribute defines the XML namespace (required for standalone SVG)
            /// Example: xmlns="http://www.w3.org/2000/svg"
            xmlns: String,
            /// The fill attribute specifies the fill color
            /// Example: fill="#3498db" (blue fill)
            fill: String,
            /// The stroke attribute specifies the outline color
            /// Example: stroke="#e74c3c" (red outline)
            stroke: String,
            /// The stroke-width attribute specifies the width of the outline
            /// Example: stroke-width="3" (3 units thick)
            stroke_width: String,
            /// The stroke-linecap attribute specifies line end style
            /// Example: stroke-linecap="round" (rounded ends)
            stroke_linecap: String,
            /// The stroke-linejoin attribute specifies how line joins are rendered
            /// Example: stroke-linejoin="miter" (pointed corners)
            stroke_linejoin: String,
            /// The stroke-miterlimit attribute limits the length of miters
            /// Example: stroke-miterlimit="4" (limits pointy corners)
            stroke_miterlimit: String,
            /// The stroke-dasharray attribute creates dashed lines
            /// Example: stroke-dasharray="5,5" (5 units on, 5 units off)
            stroke_dasharray: String,
            /// The stroke-dashoffset attribute adjusts dash pattern start
            /// Example: stroke-dashoffset="10" (starts 10 units into pattern)
            stroke_dashoffset: String,
            /// The stroke-opacity attribute sets stroke transparency
            /// Example: stroke-opacity="0.5" (50% transparent)
            stroke_opacity: String,
            /// The fill-opacity attribute sets fill transparency
            /// Example: fill-opacity="0.7" (70% opaque)
            fill_opacity: String,
        }

        /// HTML `<path>` element - Defines a path in SVG graphics
        ///
        /// Example:
        ///
        /// ```<path d="M10 10 H 90 V 90 H 10 Z" fill="transparent" stroke="black" />```
        path {
            /// The d attribute defines the path to be drawn
            /// Example: d="M20,20 L80,20 L80,80 L20,80 Z" (square path)
            d: String,
            /// The fill attribute specifies the fill color
            /// Example: fill="#3498db" (blue fill)
            fill: String,
            /// The stroke attribute specifies the outline color
            /// Example: stroke="#e74c3c" (red outline)
            stroke: String,
            /// The stroke-width attribute specifies the width of the outline
            /// Example: stroke-width="3" (3 units thick)
            stroke_width: String,
            /// The stroke-linecap attribute specifies line end style
            /// Example: stroke-linecap="round" (rounded ends)
            stroke_linecap: String,
            /// The stroke-linejoin attribute specifies how line joins are rendered
            /// Example: stroke-linejoin="miter" (pointed corners)
            stroke_linejoin: String,
            /// The stroke-miterlimit attribute limits the length of miters
            /// Example: stroke-miterlimit="4" (limits pointy corners)
            stroke_miterlimit: String,
            /// The stroke-dasharray attribute creates dashed lines
            /// Example: stroke-dasharray="5,5" (5 units on, 5 units off)
            stroke_dasharray: String,
            /// The stroke-dashoffset attribute adjusts dash pattern start
            /// Example: stroke-dashoffset="10" (starts 10 units into pattern)
            stroke_dashoffset: String,
            /// The stroke-opacity attribute sets stroke transparency
            /// Example: stroke-opacity="0.5" (50% transparent)
            stroke_opacity: String,
            /// The fill-opacity attribute sets fill transparency
            /// Example: fill-opacity="0.7" (70% opaque)
            fill_opacity: String,
        }

        /// HTML `<rect>` element - Draws a rectangle in SVG
        ///
        /// Example:
        ///
        /// ```<rect x="10" y="10" width="100" height="50" fill="blue" />```
        rect {
            /// The x attribute specifies the x-coordinate of the rectangle
            /// Example: x="25" (25 units from the left)
            x: String,
            /// The y attribute specifies the y-coordinate of the rectangle
            /// Example: y="50" (50 units from the top)
            y: String,
            /// The rx attribute specifies the horizontal corner radius
            /// Example: rx="10" (rounded corners)
            rx: String,
            /// The ry attribute specifies the vertical corner radius
            /// Example: ry="10" (rounded corners)
            ry: String,
            /// The fill attribute specifies the fill color
            /// Example: fill="#2ecc71" (green fill)
            fill: String,
            /// The stroke attribute specifies the outline color
            /// Example: stroke="#27ae60" (darker green outline)
            stroke: String,
            /// The stroke-width attribute specifies the width of the outline
            /// Example: stroke-width="2" (2 units thick)
            stroke_width: String,
        }

        /// HTML `<circle>` element - Draws a circle in SVG
        ///
        /// Example:
        ///
        /// ```<circle cx="50" cy="50" r="40" fill="red" />```
        circle {
            /// The cx attribute specifies the x-coordinate of the center
            /// Example: cx="100" (center x at 100 units)
            cx: String,
            /// The cy attribute specifies the y-coordinate of the center
            /// Example: cy="100" (center y at 100 units)
            cy: String,
            /// The r attribute specifies the radius of the circle
            /// Example: r="75" (75 units radius)
            r: String,
            /// The fill attribute specifies the fill color
            /// Example: fill="#9b59b6" (purple fill)
            fill: String,
            /// The stroke attribute specifies the outline color
            /// Example: stroke="#8e44ad" (darker purple outline)
            stroke: String,
            /// The stroke-width attribute specifies the width of the outline
            /// Example: stroke-width="3" (3 units thick)
            stroke_width: String,
        }

        /// HTML `<ellipse>` element - Draws an ellipse in SVG
        ///
        /// Example:
        ///
        /// ```<ellipse cx="100" cy="50" rx="100" ry="50" fill="yellow" />```
        ellipse {
            /// The cx attribute specifies the x-coordinate of the center
            /// Example: cx="150" (center x at 150 units)
            cx: String,
            /// The cy attribute specifies the y-coordinate of the center
            /// Example: cy="75" (center y at 75 units)
            cy: String,
            /// The rx attribute specifies the horizontal radius
            /// Example: rx="100" (100 units horizontal radius)
            rx: String,
            /// The ry attribute specifies the vertical radius
            /// Example: ry="50" (50 units vertical radius)
            ry: String,
            /// The fill attribute specifies the fill color
            /// Example: fill="#f1c40f" (yellow fill)
            fill: String,
            /// The stroke attribute specifies the outline color
            /// Example: stroke="#f39c12" (darker yellow outline)
            stroke: String,
            /// The stroke-width attribute specifies the width of the outline
            /// Example: stroke-width="2" (2 units thick)
            stroke_width: String,
        }

        /// HTML `<line>` element - Draws a line in SVG
        ///
        /// Example:
        ///
        /// ```<line x1="0" y1="0" x2="100" y2="100" stroke="black" />```
        line {
            /// The x1 attribute specifies the x-coordinate of the start point
            /// Example: x1="10" (starts 10 units from left)
            x1: String,
            /// The y1 attribute specifies the y-coordinate of the start point
            /// Example: y1="10" (starts 10 units from top)
            y1: String,
            /// The x2 attribute specifies the x-coordinate of the end point
            /// Example: x2="200" (ends 200 units from left)
            x2: String,
            /// The y2 attribute specifies the y-coordinate of the end point
            /// Example: y2="200" (ends 200 units from top)
            y2: String,
            /// The stroke attribute specifies the line color
            /// Example: stroke="#34495e" (dark blue line)
            stroke: String,
            /// The stroke-width attribute specifies the width of the line
            /// Example: stroke-width="5" (5 units thick)
            stroke_width: String,
            /// The stroke-linecap attribute specifies line end style
            /// Example: stroke-linecap="round" (rounded ends)
            stroke_linecap: String,
            /// The stroke-dasharray attribute creates dashed lines
            /// Example: stroke-dasharray="10,5" (10 units on, 5 units off)
            stroke_dasharray: String,
        }

        /// HTML `<polyline>` element - Draws connected straight lines in SVG
        ///
        /// Example:
        ///
        /// ```<polyline points="20,20 40,25 60,40 80,120 120,140 200,180" stroke="orange" fill="none" />```
        polyline {
            /// The points attribute specifies coordinates for each point
            /// Example: points="0,0 50,50 100,25" (series of x,y pairs)
            points: String,
            /// The fill attribute specifies the fill color between lines
            /// Example: fill="none" (transparent fill)
            fill: String,
            /// The stroke attribute specifies the line color
            /// Example: stroke="#e67e22" (orange line)
            stroke: String,
            /// The stroke-width attribute specifies the width of the lines
            /// Example: stroke-width="3" (3 units thick)
            stroke_width: String,
            /// The stroke-linejoin attribute specifies how lines are joined
            /// Example: stroke-linejoin="round" (rounded corners)
            stroke_linejoin: String,
        }

        /// HTML `<polygon>` element - Draws a closed shape with straight lines in SVG
        ///
        /// Example:
        ///
        /// ```<polygon points="200,10 250,190 160,210" fill="green" />```
        polygon {
            /// The points attribute specifies coordinates for each point
            /// Example: points="50,50 150,50 100,150" (triangle coordinates)
            points: String,
            /// The fill attribute specifies the fill color of the shape
            /// Example: fill="#1abc9c" (teal fill)
            fill: String,
            /// The stroke attribute specifies the outline color
            /// Example: stroke="#16a085" (darker teal outline)
            stroke: String,
            /// The stroke-width attribute specifies the width of the outline
            /// Example: stroke-width="2" (2 units thick)
            stroke_width: String,
            /// The fill-rule attribute specifies how to fill shapes with holes
            /// Example: fill-rule="evenodd" (alternates fill for nested shapes)
            fill_rule: String,
        }

        /// HTML `<g>` element - Groups SVG elements together
        ///
        /// Example:
        ///
        /// ```<g transform="rotate(45 50 50)"><rect x="20" y="20" width="60" height="60" /></g>```
        g {
            /// The transform attribute applies transformations to the group
            /// Example: transform="translate(100,50) scale(2)" (moves and scales)
            transform: String,
            /// The fill attribute specifies the fill color for all elements in the group
            /// Example: fill="#3498db" (blue fill for all children)
            fill: String,
            /// The stroke attribute specifies the outline color for all elements in the group
            /// Example: stroke="#2980b9" (darker blue outline for all children)
            stroke: String,
        }

        /// HTML `<use>` element - Reuses an SVG element defined elsewhere
        ///
        /// Example:
        ///
        /// ```<r#use href="#myCircle" x="10" y="10" fill="blue" />```
        r#use {
            /// The href attribute specifies which element to reuse
            /// Example: href="#icon-star" (references element with id="icon-star")
            href: String,
            /// The x attribute specifies the x-coordinate where to place the reused element
            /// Example: x="100" (100 units from left)
            x: String,
            /// The y attribute specifies the y-coordinate where to place the reused element
            /// Example: y="50" (50 units from top)
            y: String,
        }

        /// HTML `<foreignObject>` element - Includes non-SVG elements inside SVG
        ///
        /// Example:
        ///
        /// ```<foreignObject x="20" y="20" width="160" height="160"><div>HTML content inside SVG</div></foreignObject>```
        foreignObject {
            /// The x attribute specifies the x-coordinate of the foreign object
            /// Example: x="25" (25 units from left)
            x: String,
            /// The y attribute specifies the y-coordinate of the foreign object
            /// Example: y="25" (25 units from top)
            y: String,
        }

        /// HTML `<defs>` element - Container for reusable SVG elements
        ///
        /// Example:
        ///
        /// ```<defs><circle id="myCircle" cx="5" cy="5" r="4" /></defs>```
        defs {
        }

        /// HTML `<linearGradient>` element - Defines a linear gradient for SVG fills
        ///
        /// Example:
        ///
        /// ```<linearGradient id="grad1" x1="0%" y1="0%" x2="100%" y2="0%"><stop offset="0%" style="stop-color:rgb(255,255,0)" /></linearGradient>```
        linearGradient {
            /// The x1 attribute defines the start point of the gradient (x-coordinate)
            /// Example: x1="0%" (starts at left edge)
            x1: String,
            /// The y1 attribute defines the start point of the gradient (y-coordinate)
            /// Example: y1="0%" (starts at top edge)
            y1: String,
            /// The x2 attribute defines the end point of the gradient (x-coordinate)
            /// Example: x2="100%" (ends at right edge)
            x2: String,
            /// The y2 attribute defines the end point of the gradient (y-coordinate)
            /// Example: y2="100%" (ends at bottom edge)
            y2: String,
            /// The gradientUnits attribute defines the coordinate system for the gradient
            /// Example: gradientUnits="userSpaceOnUse" (uses absolute coordinates)
            gradientUnits: String,
            /// The spreadMethod attribute defines how the gradient fills beyond its bounds
            /// Example: spreadMethod="reflect" (gradient reflects at boundaries)
            spreadMethod: String,
        }

        /// HTML `<stop>` element - Defines color transitions in gradients
        ///
        /// Example:
        ///
        /// ```<stop offset="0%" style="stop-color:rgb(255,0,0);stop-opacity:1" />```
        stop {
            /// The offset attribute defines where along the gradient this color appears
            /// Example: offset="50%" (color positioned halfway through gradient)
            offset: String,
            /// The stop-color attribute defines the color at this stop
            /// Example: stop-color="#3498db" (blue color)
            stop_color: String,
            /// The stop-opacity attribute defines the opacity at this stop
            /// Example: stop-opacity="0.5" (50% transparent)
            stop_opacity: String,
        }

        /// HTML `<radialGradient>` element - Defines a radial gradient for SVG fills
        ///
        /// Example:
        ///
        /// ```<radialGradient id="grad2" cx="50%" cy="50%" r="50%"><stop offset="0%" style="stop-color:red" /></radialGradient>```
        radialGradient {
            /// The cx attribute defines the x-coordinate of the center point
            /// Example: cx="50%" (center of the area horizontally)
            cx: String,
            /// The cy attribute defines the y-coordinate of the center point
            /// Example: cy="50%" (center of the area vertically)
            cy: String,
            /// The r attribute defines the radius of the gradient
            /// Example: r="75%" (extends to 75% of the reference area)
            r: String,
            /// The fx attribute defines the x-coordinate of the focal point
            /// Example: fx="60%" (focal point slightly right of center)
            fx: String,
            /// The fy attribute defines the y-coordinate of the focal point
            /// Example: fy="40%" (focal point slightly above center)
            fy: String,
            /// The fr attribute defines the radius of the focal point
            /// Example: fr="5%" (small focal point)
            fr: String,
            /// The gradientUnits attribute defines the coordinate system for the gradient
            /// Example: gradientUnits="objectBoundingBox" (relative to object)
            gradientUnits: String,
            /// The spreadMethod attribute defines how the gradient fills beyond its bounds
            /// Example: spreadMethod="pad" (uses edge color beyond boundaries)
            spreadMethod: String,
        }

        /// HTML `<mask>` element - Defines an area where SVG elements are partially or fully hidden
        ///
        /// Example:
        ///
        /// ```<mask id="myMask"><rect width="100%" height="100%" fill="white" opacity="0.5" /></mask>```
        mask {
            /// The maskUnits attribute specifies the coordinate system for mask positioning
            /// Example: maskUnits="userSpaceOnUse" (absolute coordinates)
            mask_units: String,
            /// The maskContentUnits attribute specifies the coordinate system for mask content
            /// Example: maskContentUnits="objectBoundingBox" (relative to object)
            mask_content_units: String,
            /// The x attribute specifies the x-coordinate of the mask
            /// Example: x="0" (starts at left edge)
            x: String,
            /// The y attribute specifies the y-coordinate of the mask
            /// Example: y="0" (starts at top edge)
            y: String,
        }
        /// HTML `<article>` element - Defines an independent, self-contained content
        ///
        /// Example:
        ///
        /// ```<article><h1>Article Title</h1><p>Article content goes here...</p></article>```
        article {
        }

        /// HTML `<aside>` element - Defines content aside from the page content
        ///
        /// Example:
        ///
        /// ```<aside><p>Sidebar content goes here...</p></aside>```
        aside {
        }

        /// HTML `<details>` element - Defines additional details that the user can view or hide
        ///
        /// Example:
        ///
        /// ```<details><summary>Click to view details</summary><p>Details content goes here...</p></details>```
        details {
        }

        /// HTML `<figcaption>` element - Defines a caption for a `<figure>` element
        ///
        /// Example:
        ///
        /// ```<figure><img src="image.jpg" alt="Image description" /><figcaption>Caption goes here...</figcaption></figure>```
        figcaption {
        }

        /// HTML `<figure>` element - Specifies self-contained content, like illustrations, diagrams, photos, code listings, etc.
        ///
        /// Example:
        ///
        /// ```<figure><img src="image.jpg" alt="Image description" /><figcaption>Caption goes here...</figcaption></figure>```
        figure {
        }

        /// HTML `<footer>` element - Defines a footer for a document or section
        ///
        /// Example:
        ///
        /// ```<footer><p>Footer content goes here...</p></footer>```
        footer {
        }

        /// HTML `<header>` element - Defines a container for introductory content or a set of navigational links
        ///
        /// Example:
        ///
        /// ```<header><h1>Header content goes here...</h1></header>```
        header {
        }

        /// HTML `<main>` element - Specifies the main content of a document
        ///
        /// Example:
        ///
        /// ```<main><p>Main content goes here...</p></main>```
        main {
        }

        /// HTML `<mark>` element - Defines marked/highlighted text
        ///
        /// Example:
        ///
        /// ```<p>Some <mark>marked</mark> text.</p>```
        mark {
        }

        /// HTML `<nav>` element - Defines navigation links
        ///
        /// Example:
        ///
        /// ```<nav><a href="#home">Home</a> <a href="#about">About</a> <a href="#contact">Contact</a></nav>```
        nav {
        }

        /// HTML `<section>` element - Defines a section in a document
        ///
        /// Example:
        ///
        /// ```<section><h2>Section Title</h2><p>Section content goes here...</p></section>```
        section {
        }

        /// HTML `<summary>` element - Defines a visible heading for a `<details>` element
        ///
        /// Example:
        ///
        /// ```<details><summary>Click to view details</summary><p>Details content goes here...</p></details>```
        summary {
        }

        /// HTML `<time>` element - Defines a date/time
        ///
        /// Example:
        ///
        /// ```<p>Published on <time datetime="2023-08-01">August 1, 2023</time></p>```
        time {
            /// The datetime attribute specifies the date/time
            /// Example: datetime="2023-08-01" (August 1, 2023)
            datetime: String,
            /// The pubdate attribute specifies that the content is published
            /// Example: pubdate="pubdate" (content is published)
            pubdate: String,
        }

        /// HTML `<wbr>` element - Inserts a line break opportunity
        ///
        /// Example:
        ///
        /// ```<p>Here is a long word:<wbr>supercalifragilisticexpialidocious</p>```
        wbr {
        }

        /// HTML `<address>` element - Defines contact information for the author/owner of a document
        ///
        /// Example:
        ///
        /// ```<address>Contact us at <a href="mailto:="mail="mailto:EMAIL"EMAILample.com</a></address>```
        address {
        }

        /// HTML `<bdi>` element - Defines a term/name within a description list
        ///
        /// Example:
        ///
        /// ```<dl><dt><bdi>Name</bdi></dt><dd>John Doe</dd></dl>```
        bdi {
        }

        /// HTML `<bdo>` element - Overrides the current text direction
        ///
        /// Example:
        ///
        /// ```<p><bdo dir="rtl">This text is written from right to left</bdo></p>```
        bdo {
        }

        /// HTML `<cite>` element - Defines the title of a work
        ///
        /// Example:
        ///
        /// ```<p>To learn more about <cite>HTML</cite>, visit <a href="```<p>To learn more about <cite>HTML</cite>, visit <a href="URL_ADDRESS.w3.org/TR/html52/">W3C</a>.</p>```
        cite {
        }

        /// HTML `<dfn>` element - Defines a definition
        ///
        /// Example:
        ///
        /// ```<p>The <dfn>HTML</dfn> element is used to define a section in a document.</p>```
        dfn {
        }

        /// HTML `<em>` element - Defines emphasized text
        ///
        /// Example:
        ///
        /// ```<p>He is <em>very</em> angry.</p>```
        em {
        }

        /// HTML `<i>` element - Defines a part of text in an alternate voice or mood
        ///
        /// Example:
        ///
        /// ```<p><i>This text is in italics.</i></p>```
        i {
        }

        /// HTML `<kbd>` element - Defines keyboard input
        ///
        /// Example:
        ///
        /// ```<p>Press <kbd>Ctrl</kbd> + <kbd>C</kbd> to copy.</p>```
        kbd {
        }

        /// HTML `<meter>` element - Defines a scalar measurement within a known range (a gauge)
        ///
        /// Example:
        ///
        /// ```<meter value="75" min="0" max="100"></meter>```
        meter {
            /// The value attribute specifies the current value
            /// Example: value="75" (current value is 75)
            value: String,
            /// The min attribute specifies the minimum value
            /// Example: min="0" (minimum value is 0)
            min: String,
            /// The max attribute specifies the maximum value
            /// Example: max="100" (maximum value is 100)
            max: String,
            /// The low attribute specifies the low value
            /// Example: low="33" (low value is 33)
            low: String,
            /// The high attribute specifies the high value
            /// Example: high="66" (high value is 66)
            high: String,
            /// The optimum attribute specifies the optimum value
            /// Example: optimum="50" (optimum value is 50)
            optimum: String,
        }

        /// HTML `<output>` element - Defines the result of a calculation
        ///
        /// Example:
        ///
        /// ```<form><input type="number" id="num1" /><input type="number" id="num2" /><button>Calculate</button><output id="result"></output></form>```
        output {
        }

        /// HTML `<progress>` element - Defines the progress of a task
        ///
        /// Example:
        ///
        /// ```<progress value="75" max="100"></progress>```
        progress {
            /// The value attribute specifies the current value
            /// Example: value="75" (current value is 75)
            value: String,
            /// The max attribute specifies the maximum value
            /// Example: max="100" (maximum value is 100)
            max: String,
        }

        /// HTML `<q>` element - Defines a short inline quotation
        ///
        /// Example:
        ///
        /// ```<p><q>We are the so-called "Vikings" from the north.</q></p>```
        q {
        }

        /// HTML `<rp>` element - Defines a parenthesis for browsers that do not support `<ruby>`
        ///
        /// Example:
        ///
        /// ```<ruby><rb>漢</rb><rp>(</rp><rt>han</rt><rp>)</rp></ruby>```
        rp {
        }

        /// HTML `<rt>` element - Defines a ruby text
        ///
        /// Example:
        ///
        /// ```<ruby><rb>漢</rb><rt>han</rt></ruby>```
        rt {
        }

        /// HTML `<ruby>` element - Defines a ruby annotation (for East Asian typography)
        ///
        /// Example:
        ///
        /// ```<ruby><rb>漢</rb><rt>han</rt></ruby>```
        ruby {
        }

        /// HTML `<s>` element - Defines strikethrough text
        ///
        /// Example:
        ///
        /// ```<p>Price: <s>$100</s> $50</p>```
        s {
        }

        /// HTML `<samp>` element - Defines sample output from a computer program
        ///
        /// Example:
        ///
        /// ```<samp>Hello, World!</samp>```
        samp {
        }

        /// HTML `<small>` element - Defines smaller text
        ///
        /// Example:
        ///
        /// ```<p><small>This is some smaller text.</small></p>```
        small {
        }

        /// HTML `<strong>` element - Defines important text
        ///
        /// Example:
        ///
        /// ```<p><strong>This is important!</strong></p>```
        strong {
        }

        /// HTML `<sub>` element - Defines subscript text
        ///
        /// Example:
        ///
        /// ```<p>H<sub>2</sub>O</p>```
        sub {
        }

        /// HTML `<sup>` element - Defines superscript text
        ///
        /// Example:
        ///
        /// ```<p>X<sup>2</sup></p>```
        sup {
        }

        /// HTML `<var>` element - Defines a variable
        ///
        /// Example:
        ///
        /// ```<p>The area is <var>x</var> times <var>y</var>.</p>```
        var {
        }

        /// HTML `<template>` element - Defines a container for content that is not to be rendered when a page is loaded
        ///
        /// Example:
        ///
        /// ```<template><p>This content will not be rendered.</p></template>```
        template {
        }

        /// HTML `<u>` element - Defines text that should be rendered as underlined
        ///
        /// Example:
        ///
        /// ```<p><u>This text is underlined.</u></p>```
        u {
        }

        /// HTML `<noscript>` element - Defines content that is displayed to users with disabled scripts
        ///
        /// Example:
        ///
        /// ```<noscript><p>JavaScript is disabled.</p></noscript>```
        noscript {
        }

        /// HTML `<legend>` element - Defines a caption for a `<fieldset>` element
        ///
        /// Example:
        ///
        /// ```<fieldset><legend>Personal Information</legend><input type="text" name="name" /><input type="email" name="email" /></fieldset>```
        legend {
        }

        /// HTML `<optgroup>` element - Defines a group of related `<option>` elements in a `<select>` element
        ///
        /// Example:
        ///
        /// ```<select><optgroup label="Fruits"><option value="apple">Apple</option><option value="banana">Banana</option></optgroup><optgroup label="Vegetables"><option value="carrot">Carrot</option><option value="potato">Potato</option></optgroup></select>```
        optgroup {
            /// The label attribute specifies a label for the group
            /// Example: label="Fruits" (label for the group is "Fruits")
            label: String,
        }

        /// HTML `<dialog>` element - Defines a dialog box or other interactive component
        ///
        /// Example:
        ///
        /// ```<dialog><p>Dialog content goes here...</p><button>Close</button></dialog>```
        dialog {
            /// The open attribute specifies whether the dialog is open or closed
            /// Example: open="open" (dialog is open)
            open: String,
        }

        /// HTML `<blockquote>` element - Defines a section that is quoted from another source
        ///
        /// Example:
        ///
        /// ```<blockquote><p>This is a quote.</p><footer>- John Doe</footer></blockquote>```
        blockquote {
        }

        /// HTML `<dd>` element - Defines a description/value pair within a `<dl>` element
        ///
        /// Example:
        ///
        /// ```<dl><dt>Coffee</dt><dd>Black hot drink</dd><dt>Milk</dt><dd>White cold drink</dd></dl>```
        dd {
        }

        /// HTML `<dl>` element - Defines a description list
        ///
        /// Example:
        ///
        /// ```<dl><dt>Coffee</dt><dd>Black hot drink</dd><dt>Milk</dt><dd>White cold drink</dd></dl>```
        dl {
        }

        /// HTML `<dt>` element - Defines a term/name in a description list
        ///
        /// Example:
        ///
        /// ```<dl><dt>Coffee</dt><dd>Black hot drink</dd><dt>Milk</dt><dd>White cold drink</dd></dl>```
        dt {
        }

        /// HTML `<base>` element - Specifies the base URL/target for all relative URLs in a document
        ///
        /// Example:
        ///
        /// ```<base href="```<base href="URL_ADDRESS.example.com/" target="_blank">```
        base {
            /// The href attribute specifies the base URL
            /// ```<base href="URL_ADDR```<base href="URL_ADDRESS.example.com/" target="_blank">```
            href: String,
            /// The target attribute specifies the target for all relative URLs
            /// ```<base href="URL_ADDRESS.example.com/" target="_blank">```
            target: String,
        }
    }
}
