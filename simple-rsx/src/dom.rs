use crate::{Component, Node, signals::run_scope};

#[cfg(feature = "wasm")]
pub fn render_root<C: Component>(element_id: &'static str)
where
    <C as Component>::Props: Default,
    <C as Component>::Props: 'static,
{
    loop {
        render_component::<C>(
            || Default::default(),
            move |node| {
                let window = web_sys::window().expect("no global `window` exists");
                let document = window.document().expect("should have a document on window");
                let mount_point = document
                    .get_element_by_id(&element_id)
                    .expect("couldn't find element");
                mount_point.set_inner_html(&node.to_string());
            },
        );
    }
}

pub fn render_component<C: Component>(
    props_fn: impl FnOnce() -> C::Props + 'static,
    callback: impl Fn(&Node) + 'static,
) -> Option<Node>
where
    <C as Component>::Props: 'static,
{
    run_scope(move || C::render(props_fn()), callback)
}
