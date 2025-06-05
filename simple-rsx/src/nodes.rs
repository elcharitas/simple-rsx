//! Simple RSX - A React-inspired JSX Library for Rust (no_std)
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
//! - **No-std Support**: Works in embedded and resource-constrained environments
//!
//! # Let's Get Started!
//!
//! Here's a quick taste of what you can do:
//!
//! ```rust
//! extern crate alloc;
//! use simple_rsx::prelude::*;
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
//! extern crate alloc;
//! use simple_rsx::prelude::*;
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
//! extern crate alloc;
//! use simple_rsx::prelude::*;
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
//! // Conditional rendering? Use the when! macro
//! let show = true;
//! let conditional = when!(show =>
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
//!         {items.iter().map(|item| {
//!             let item = item.to_string();
//!             rsx!(<li>{item}</li>)
//!         })}
//!     </ul>
//! );
//! ```
//!
//! ## Components and Props - Build Reusable UI
//!
//! ```rust
//! extern crate alloc;
//! use simple_rsx::prelude::*;
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
//! fn Button(props: &ButtonProps) -> Node {
//!     rsx!(
//!         <button class={format!("btn btn-{}", props.variant)}>
//!             {&props.text}
//!             {&props.children}
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
//! ```rust ignore
//! extern crate alloc;
//! use simple_rsx::prelude::*;
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

extern crate alloc;

// For no_std, we need to use alloc collections instead of std
use crate::signals::{Signal, SignalValue};
use alloc::{
    borrow::Cow,
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use core::{fmt::Display, iter::FromIterator};

pub use simple_rsx_macros::{component, rsx, when};

#[cfg(feature = "wasm")]
use alloc::{boxed::Box, sync::Arc};

/// A trait for converting values into HTML attribute strings.
///
/// This trait is automatically implemented for any type that implements `ToString`,
/// making it easy to use various types as attribute values.
///
/// # Example
///
/// ```rust
/// use simple_rsx::prelude::*;
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
/// use simple_rsx::prelude::*;
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

#[derive(Clone)]
/// Represents an HTML element with its tag name, attributes, and children.
///
/// Elements are the building blocks of the RSX tree structure. Each element
/// can have attributes (like class, id, etc.) and can contain other elements
/// or text nodes as children.
///
/// You typically won't create Elements directly, but rather use the `rsx!` macro:
///
/// ```rust
/// use simple_rsx::prelude::*;
///
/// let element = rsx!(
///     <div class="container">
///         <p>Hello world!</p>
///     </div>
/// );
/// ```
pub struct Element {
    pub(crate) key: String,
    tag: Cow<'static, str>,
    attributes: BTreeMap<String, String>,
    children: Vec<Node>,
    #[cfg(feature = "wasm")]
    events: BTreeMap<String, EventCallback>,
    #[cfg(not(feature = "wasm"))]
    #[allow(unused)]
    events: BTreeMap<String, String>,
}

impl Element {
    pub fn parse_tag_with_attributes(
        key: &str,
        tag: &'static str,
        attributes: BTreeMap<String, String>,
        #[cfg(feature = "wasm")] events: BTreeMap<String, EventCallback>,
        #[cfg(not(feature = "wasm"))] events: BTreeMap<String, String>,
        children: Vec<Node>,
    ) -> Node {
        Node::Element(Element {
            tag: Cow::Borrowed(tag),
            key: key.to_string(),
            attributes,
            events,
            children,
        })
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }

    pub fn children(&self) -> &Vec<Node> {
        &self.children
    }

    #[cfg(not(feature = "wasm"))]
    pub fn events(&self) -> &BTreeMap<String, String> {
        &self.events
    }

    #[cfg(feature = "wasm")]
    pub fn events(&self) -> &BTreeMap<String, EventCallback> {
        &self.events
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
/// use simple_rsx::prelude::*;
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
///     fn render(props: &Self::Props) -> Node {
///         rsx!(
///             <div class="card">
///                 <h2>{&props.title}</h2>
///                 <div class="card-content">{&props.children}</div>
///             </div>
///         )
///     }
/// }
/// ```
pub trait Component {
    /// The type of props this component accepts
    type Props;

    /// Renders the component with the given props
    fn render(props: &Self::Props) -> Node;
}

#[derive(Default)]
pub struct DefaultProps;

#[derive(Clone)]
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
/// use simple_rsx::prelude::*;
///
/// let text_node = Node::Text("Hello".to_string());
/// let fragment = Node::Fragment(vec![text_node]);
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

    /// Attempts to get a reference to the underlying Element if this node is an Element.
    ///
    /// Returns None if the node is not an Element (e.g., if it's Text or Fragment).
    pub fn as_element(&self) -> Option<&Element> {
        match self {
            Node::Element(el) => Some(el),
            _ => None,
        }
    }
}

impl From<String> for Node {
    fn from(value: String) -> Self {
        Node::Text(value.into())
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

impl FromIterator<Node> for Node {
    fn from_iter<T: IntoIterator<Item = Node>>(iter: T) -> Self {
        Node::Fragment(iter.into_iter().collect())
    }
}

impl<T: Clone> From<&Vec<T>> for Node
where
    Node: From<T>,
{
    fn from(value: &Vec<T>) -> Self {
        Node::Fragment(value.clone().into_iter().map(|t| Node::from(t)).collect())
    }
}

impl<T> From<Option<T>> for Node
where
    Node: From<T>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(t) => Node::from(t),
            _ => Node::Empty,
        }
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

impl<T: SignalValue + PartialEq + Clone + 'static> From<Signal<T>> for Node
where
    T: Into<Node>,
{
    fn from(value: Signal<T>) -> Self {
        value.get().into()
    }
}

impl<I, F, R> From<core::iter::Map<I, F>> for Node
where
    I: Iterator,
    F: FnMut(I::Item) -> R,
    R: Into<Node>,
    Vec<Node>: FromIterator<R>,
{
    fn from(iter: core::iter::Map<I, F>) -> Self {
        let nodes: Vec<Node> = iter.collect();
        Node::from(nodes)
    }
}

impl<I, F, R> From<&core::iter::Map<I, F>> for Node
where
    I: Iterator + Clone,
    F: FnMut(I::Item) -> R + Clone,
    R: Into<Node>,
    Vec<Node>: FromIterator<R>,
{
    fn from(iter: &core::iter::Map<I, F>) -> Self {
        let nodes: Vec<Node> = iter.clone().collect();
        Node::from(nodes)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

#[cfg(feature = "wasm")]
pub struct EventCallback(Option<Arc<spin::Mutex<Box<dyn FnMut(web_sys::Event) + Send + Sync>>>>);

#[cfg(feature = "wasm")]
impl Default for EventCallback {
    fn default() -> Self {
        Self(None)
    }
}

#[cfg(feature = "wasm")]
impl EventCallback {
    pub fn new<F>(callback: F) -> Self
    where
        F: FnMut(web_sys::Event) + Send + Sync + 'static,
    {
        Self(Some(Arc::new(spin::Mutex::new(Box::new(callback)))))
    }

    pub fn has_callback(&self) -> bool {
        self.0.is_some()
    }

    pub fn call(&mut self, event: web_sys::Event) {
        if let Some(cb) = &mut self.0 {
            let mut cb = cb.lock();
            cb(event);
        }
    }
}

#[cfg(feature = "wasm")]
impl Clone for EventCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

// For convenience with Fn closures
#[cfg(feature = "wasm")]
impl<F> From<F> for EventCallback
where
    F: FnMut(web_sys::Event) + Send + Sync + 'static,
{
    fn from(callback: F) -> Self {
        Self::new(callback)
    }
}
