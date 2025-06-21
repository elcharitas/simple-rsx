// For no_std, we need to use alloc collections instead of std
use crate::signals::{Signal, SignalValue};
use alloc::{
    borrow::Cow,
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use core::{fmt::Display, iter::FromIterator};

pub use momenta_macros::{component, rsx, when};

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
/// use momenta::prelude::*;
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
/// use momenta::prelude::*;
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
/// use momenta::prelude::*;
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
    inner_html: String,
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
        inner_html: &str,
        children: Vec<Node>,
    ) -> Node {
        Node::Element(Element {
            tag: Cow::Borrowed(tag),
            key: key.to_string(),
            attributes,
            events,
            children,
            inner_html: inner_html.to_string(),
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

    pub(crate) fn html(&self) -> &String {
        &self.inner_html
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
/// use momenta::prelude::*;
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
/// use momenta::prelude::*;
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
                if el.children.is_empty() && !el.html().is_empty() {
                    write!(f, "{}", el.html())?;
                } else {
                    for child in &el.children {
                        write!(f, "{}", child)?;
                    }
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
