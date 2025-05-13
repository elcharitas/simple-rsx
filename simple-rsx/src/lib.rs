use indexmap::IndexMap;
pub use simple_rsx_macros::rsx;
use std::fmt::Display;

/// A trait for attributes
pub trait Attribute {
    fn value(&self) -> String;
}

impl<T: ToString> Attribute for T {
    fn value(&self) -> String {
        self.to_string()
    }
}

#[derive(Clone)]
pub struct Element {
    tag: String,
    attributes: IndexMap<String, String>,
    children: Vec<Node>,
}

impl Element {
    pub fn new(tag: &str) -> Node {
        Node::Element(Element {
            tag: tag.to_string(),
            attributes: IndexMap::new(),
            children: Vec::new(),
        })
    }

    pub fn set_attribute(&mut self, name: &str, value: impl Attribute) {
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

#[derive(Clone)]
pub enum Node {
    Element(Element),
    Text(String),
    Fragment(Vec<Node>),
    Comment(String),
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

impl<T: ToString> From<Vec<T>> for Node {
    fn from(value: Vec<T>) -> Self {
        Node::Fragment(
            value
                .into_iter()
                .map(|t| Node::Text(t.to_string()))
                .collect(),
        )
    }
}

impl<T: ToString> From<Option<T>> for Node {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(t) => Node::Text(t.to_string()),
            None => Node::Text("".to_string()),
        }
    }
}

impl From<&Vec<String>> for Node {
    fn from(value: &Vec<String>) -> Self {
        Node::Fragment(value.iter().map(|s| Node::Text(s.to_string())).collect())
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
        for i in iter {
            result.push(Node::Text(i.to_string()));
        }
        Node::Fragment(result)
    }
}

impl FromIterator<u64> for Node {
    fn from_iter<T: IntoIterator<Item = u64>>(iter: T) -> Self {
        let mut result = Vec::new();
        for i in iter {
            result.push(Node::Text(i.to_string()));
        }
        Node::Fragment(result)
    }
}

impl FromIterator<i32> for Node {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        let mut result = Vec::new();
        for i in iter {
            result.push(Node::Text(i.to_string()));
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
        }
    }
}
