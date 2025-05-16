//! Counter example demonstrating WASM-based hooks usage in Simple RSX

use simple_rsx::*;
use wasm_bindgen::prelude::*;

#[derive(Default)]
struct CounterProps {
    initial: i32,
    children: Vec<Node>,
}

#[component]
fn Counter(props: CounterProps) -> Node {
    let count = props.initial;
    let double_count = 1;

    rsx!(
        <div class="counter">
            <h2>Counter Example</h2>
            <p>Count: {count}</p>
            <p>Double Count: {double_count}</p>
            <div class="buttons">
                <button>+</button>
                <button>-</button>
            </div>
            {props.children}
        </div>
    )
}

#[wasm_bindgen(start)]
pub fn run() {
    let counter = rsx!(<Counter initial={0} />);
    // Mount the counter to a container element
    let document = web_sys::window().unwrap().document().unwrap();
    let container = document.get_element_by_id("app").unwrap();
    container.set_inner_html(&counter.to_string());
}
