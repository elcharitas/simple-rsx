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
                if let Some(el) = child.as_element() {
                    if !el.key.is_empty() {
                        // cache by key
                    }
                    el.render(&element);
                }
                if let Some(text) = child.as_text() {
                    let current_text = element.text_content().unwrap_or_default();
                    let _ = element.set_text_content(Some(&(current_text + text)));
                }
            }
            return Some(element);
        }
        None
    }
}

#[cfg(feature = "wasm")]
impl WasmRender for Node {
    fn render(&self, mount: &web_sys::Element) -> Option<web_sys::Element> {
        if let Some(el) = self.as_element() {
            return el.render(mount);
        }
        None
    }
}

#[cfg(feature = "wasm")]
pub fn render_root<C: Component>(element_id: &'static str)
where
    <C as Component>::Props: Default,
    <C as Component>::Props: Clone + 'static,
{
    render_component::<C>(Default::default(), move |node| {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let mount_point = document
            .get_element_by_id(&element_id)
            .expect("couldn't find element");
        // clear mount point
        while let Some(child) = mount_point.first_child() {
            mount_point.remove_child(&child).ok();
        }
        node.render(&mount_point);
    });
}

#[cfg(feature = "wasm")]
pub fn attach_event_handler(
    element: &web_sys::Element,
    event_type: &str,
    mut callback: crate::EventCallback,
) {
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
    callback: impl Fn(&Node) + 'static,
) -> Option<Node>
where
    <C as Component>::Props: 'static,
{
    run_scope(move || C::render(&props), callback)
}
