use crate::{Component, Node, signals::run_scope};

#[cfg(feature = "wasm")]
pub trait WasmRender {
    fn render(&self, mount: &web_sys::Element) -> Option<web_sys::Element>;
}

#[cfg(feature = "wasm")]
impl WasmRender for crate::Element {
    fn render(&self, mount: &web_sys::Element) -> Option<web_sys::Element> {
        let element = web_sys::window()
            .map(|window| window.document().map(|doc| doc.create_element(&self.tag)))
            .flatten()?;
        if let Ok(element) = element {
            let _ = mount.append_child(&element);
            for child in &self.children {
                child.render(&element);
            }
            // add attributes
            for (name, value) in &self.attributes {
                let _ = element.set_attribute(name, value);
            }
            // attach events
            for (event_type, callback) in &self.events {
                attach_event_handler(&element, event_type, callback.clone());
            }
            return Some(element);
        }
        None
    }
}

#[cfg(feature = "wasm")]
impl WasmRender for Node {
    fn render(&self, mount: &web_sys::Element) -> Option<web_sys::Element> {
        match self {
            Node::Text(text) => {
                let current_text = mount.text_content().unwrap_or_default();
                mount.set_text_content(Some(&(current_text + text)));
                return None;
            }
            Node::Element(el) => {
                return el.render(mount);
            }
            Node::Fragment(fragment) => {
                for child in fragment {
                    child.render(mount);
                }
                return None;
            }
            Node::Comment(_) => {
                // TODO: implement comment rendering
                return None;
            }
            _ => None,
        }
    }
}

#[cfg(feature = "wasm")]
pub fn render_root<C: Component>(selectors: &'static str)
where
    <C as Component>::Props: Default,
    <C as Component>::Props: Send + Sync + 'static,
{
    render_component::<C>(Default::default(), move |node| {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let mount_point = document
            .query_selector(&selectors)
            .expect("couldn't find element")
            .expect("couldn't find element");
        // clear mount point
        while let Some(child) = mount_point.first_child() {
            mount_point.remove_child(&child).ok();
        }
        node.render(&mount_point);
    });
}

#[cfg(feature = "wasm")]
pub fn mount_to_body<C: Component>()
where
    <C as Component>::Props: Default,
    <C as Component>::Props: Send + Sync + 'static,
{
    render_root::<C>("body");
}

#[cfg(feature = "wasm")]
fn attach_event_handler(
    element: &web_sys::Element,
    event_type: &str,
    mut callback: crate::EventCallback,
) {
    use alloc::boxed::Box;
    use wasm_bindgen::prelude::*;

    let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
        callback.call(event);
    }) as Box<dyn FnMut(web_sys::Event)>);

    element
        .add_event_listener_with_callback(event_type, closure.as_ref().unchecked_ref())
        .expect("Failed to add event listener");

    closure.forget(); // Keep the closure alive
}

pub fn render_component<C: Component>(
    props: C::Props,
    callback: impl Fn(&Node) + 'static + Send + Sync,
) -> Option<Node>
where
    <C as Component>::Props: Send + Sync + 'static,
{
    run_scope(move || C::render(&props), callback)
}
