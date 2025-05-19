use crate::{Component, Node, signals::run_scope};

pub fn render_node<C: Component<Props = ()>>(mount_point: web_sys::Element) {
    loop {
        let node = render_component::<C>(()).unwrap();
        mount_point.set_inner_html(&node.to_string());
    }
}

pub fn render_component<C: Component>(props: C::Props) -> Option<Node>
where
    <C as Component>::Props: Send + Copy + Sync + 'static,
{
    run_scope(move || C::render(props))
}
