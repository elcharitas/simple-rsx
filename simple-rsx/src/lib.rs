pub use simple_rsx_macros::jsx;
use std::fmt::Display;

pub enum NodeList {
    Fragment(Vec<Node>),
    Single(Node),
}

impl Display for NodeList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeList::Fragment(nodes) => {
                for node in nodes {
                    write!(f, "{}", node)?;
                }
                Ok(())
            }
            NodeList::Single(node) => {
                write!(f, "{}", node)?;
                Ok(())
            }
        }
    }
}

pub enum Node {
    Element(Element),
    Text(String),
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
                write!(f, "{}", text)?;
                Ok(())
            }
        }
    }
}

pub trait AttributeValue {
    fn value(&self) -> String;
}

impl<T: ToString> AttributeValue for T {
    fn value(&self) -> String {
        self.to_string()
    }
}

pub struct Element {
    tag: String,
    attributes: std::collections::HashMap<String, String>,
    children: Vec<Node>,
}

impl Element {
    pub fn new(tag: &str) -> Node {
        Node::Element(Element {
            tag: tag.to_string(),
            attributes: std::collections::HashMap::new(),
            children: Vec::new(),
        })
    }

    pub fn set_attribute(&mut self, name: &str, value: impl AttributeValue) {
        self.attributes.insert(name.to_string(), value.value());
    }

    pub fn append_child(&mut self, node: Node) {
        self.children.push(node);
    }
}

impl Node {
    pub fn as_element_mut(&mut self) -> Option<&mut Element> {
        match self {
            Node::Element(el) => Some(el),
            _ => None,
        }
    }

    pub fn append_child(&mut self, node: Node) {
        if let Node::Element(el) = self {
            el.children.push(node);
        }
    }
}

pub struct TextNode {
    text: String,
}

impl TextNode {
    pub fn new(text: &str) -> Node {
        Node::Text(text.to_string())
    }
}
