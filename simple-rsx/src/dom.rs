use crate::{Component, Node, signals::run_scope};

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[cfg(feature = "wasm")]
mod element_cache {
    use alloc::{collections::BTreeMap, string::String};
    use core::cell::UnsafeCell;

    #[derive(Debug)]
    // UnsafeCell wrapper for WASM single-threaded environment
    struct ElementCache {
        inner: UnsafeCell<Option<BTreeMap<String, web_sys::Element>>>,
    }

    unsafe impl Sync for ElementCache {}

    static ELEMENT_CACHE: ElementCache = ElementCache {
        inner: UnsafeCell::new(None),
    };

    // Safe in WASM because it's single-threaded
    pub(crate) fn with_cache<F, R>(f: F) -> R
    where
        F: FnOnce(&mut BTreeMap<String, web_sys::Element>) -> R,
    {
        unsafe {
            let cache = &mut *ELEMENT_CACHE.inner.get();
            if cache.is_none() {
                *cache = Some(BTreeMap::new());
            }
            f(cache.as_mut().unwrap())
        }
    }
}

#[cfg(feature = "wasm")]
trait WasmRender {
    fn render(&self, mount: &web_sys::Element) -> Option<web_sys::Element>;
}

#[cfg(feature = "wasm")]
impl WasmRender for crate::Element {
    fn render(&self, mount: &web_sys::Element) -> Option<web_sys::Element> {
        let element = web_sys::window()
            .map(|window| window.document().map(|doc| doc.create_element(&self.tag)))
            .flatten()
            .and_then(|el| el.ok());

        if let Some(element) = element {
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

            element_cache::with_cache(|cache| {
                use alloc::string::ToString;
                cache.insert(self.key().to_string(), element.clone());
            });
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
                mount.set_text_content(Some(&(current_text + &text)));
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
/// Renders the root component to the specified selector
///
/// # Example
///
/// ```rust ignore
/// use simple_rsx::{dom::render_root, signals::create_signal, *};
///
/// #[component]
/// fn App() -> Node {
///     let count = create_signal(0);
///
///     rsx! {
///         <div>
///             <h1>Hello World</h1>
///             Count: {count}
///         </div>
///     }
/// }
///
/// fn main() {
///     render_root::<App>("#app");
/// }
/// ```
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
/// Mounts the root component to the body element
///
/// # Example
///
/// ```rust ignore
/// use simple_rsx::{dom::mount_to_body, signals::create_signal, *};
///
/// #[component]
/// fn App() -> Node {
///     let count = create_signal(0);
///
///     rsx! {
///         <div>
///             <h1>Hello World</h1>
///             Count: {count}
///         </div>
///     }
/// }
///
/// fn main() {
///     mount_to_body::<App>();
/// }
/// ```
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
    callback: impl Fn(&Node) + Send + Sync + 'static,
) -> Option<Node>
where
    <C as Component>::Props: Send + Sync + 'static,
{
    run_scope(
        move || C::render(&props),
        move |node| {
            #[cfg(feature = "wasm")]
            if let Node::Element(el) = node {
                if let Some(element) =
                    element_cache::with_cache(|cache| cache.get(el.key()).cloned())
                {
                    if let Some(parent) = element.parent_element() {
                        parent.remove_child(&element).ok();
                        el.render(&parent);
                    }
                }
            }
            callback(node)
        },
    )
}
