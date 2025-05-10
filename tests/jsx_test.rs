use std::{fmt::Display, str::FromStr};

use simple_rsx::jsx;

enum NodeList {
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

enum Node {
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

struct Element {
    tag: String,
    attributes: std::collections::HashMap<String, String>,
    children: Vec<Node>,
}

impl Element {
    fn new(tag: &str) -> Node {
        Node::Element(Element {
            tag: tag.to_string(),
            attributes: std::collections::HashMap::new(),
            children: Vec::new(),
        })
    }

    fn set_attribute(&mut self, name: &str, value: impl AttributeValue) {
        self.attributes.insert(name.to_string(), value.value());
    }

    fn append_child(&mut self, node: Node) {
        self.children.push(node);
    }
}

impl Node {
    fn as_element_mut(&mut self) -> Option<&mut Element> {
        match self {
            Node::Element(el) => Some(el),
            _ => None,
        }
    }

    fn append_child(&mut self, node: Node) {
        if let Node::Element(el) = self {
            el.children.push(node);
        }
    }
}

struct TextNode {
    text: String,
}
impl TextNode {
    fn new(text: &str) -> Node {
        Node::Text(text.to_string())
    }
}

#[test]
fn test_jsx() {
    let jsx = jsx!(<div class="container" id="app" />);
    let expected = String::from("<div class=\"container\" id=\"app\"></div>");
    let result = jsx.to_string();
    assert_eq!(result, expected); // Replace with your expected result
}

#[test]
fn test_jsx_with_children() {
    let jsx = jsx!(<div class="container" id=3><p>"Hello, world!"</p></div>);
    let expected = String::from("<div class=\"container\" id=\"app\"><p>Hello, world!</p></div>");
    let result = jsx.to_string();
    assert_eq!(result, expected); // Replace with your expected result
}
