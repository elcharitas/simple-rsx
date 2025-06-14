#![no_std]
//! Momenta - A React-inspired JSX Library for Rust (no_std)
//!
//! I created Momenta to bring the familiar feel of React's JSX to Rust projects. If you're coming
//! from a React background, you'll feel right at home. And if you're new to both, don't worry - I've made
//! it super intuitive while keeping all the type safety and performance benefits of Rust.
//!
//! # Why Momenta?
//!
//! I started this project while attempting to transit my [portfolio](https://elcharitas.wtf) from Next.js to Rust.
//! I wanted to keep my codebase as simple as possible, and I wanted to use Rust's powerful type system
//! to ensure that my components were always correct. I tried existing libraries like `yew` and `sycamore`,
//! but they were either too complex or didn't feel quite like React. And so, here we are.
//!
//! I know what you're thinking - "Another UI library?" But here's what makes Momenta special:
//!
//! - **React-like Syntax**: Write your templates using the `rsx!` macro - it's just like JSX!
//! - **Type Safety**: Get compile-time checks for your components and props
//! - **Zero Runtime Overhead**: All the magic happens at compile time
//! - **Familiar Patterns**: Components, props, fragments - all the React concepts you love
//! - **No-std Support**: Works in embedded and resource-constrained environments
//!
//! # Let's Get Started!
//!
//! Here's a quick taste of what you can do:
//!
//! ```rust
//! extern crate alloc;
//! use momenta::prelude::*;
//!
//! // Create your first component - looks familiar, right?
//! let greeting = rsx!(
//!     <div class="greeting">
//!         <h1>Hello, {"World"}!</h1>
//!         <p>Welcome to Momenta</p>
//!     </div>
//! );
//!
//! // Turn it into HTML - perfect for server-side rendering (P.S: This to me is my favorite feature)
//! println!("{}", greeting);
//! ```
//!
//! # Features You'll Love
//!
//! ## JSX-style Elements - Write HTML, Get Rust
//!
//! ```rust
//! extern crate alloc;
//! use momenta::prelude::*;
//!
//! // Self-closing tags? Check!
//! let img = rsx!(<img src="image.jpg" alt="An image" />);
//!
//! // Nested elements? Of course!
//! let card = rsx!(
//!     <div class="card">
//!         <h2>Title</h2>
//!         <p>Content</p>
//!     </div>
//! );
//!
//! // Fragments? No problem! Just use <> and the children will be flattened
//! let fragment = rsx!(
//!     <>
//!         <h1>Title</h1>
//!         <p>No wrapper needed</p>
//!     </>
//! );
//! ```
//!
//! ## Dynamic Content - Make It Come Alive
//!
//! ```rust
//! extern crate alloc;
//! use momenta::prelude::*;
//!
//! let name = "World";
//! let count = 42;
//!
//! // Drop in any Rust expression with {}
//! let dynamic = rsx!(
//!     <div>
//!         <h1>Hello, {name}!</h1>
//!         <p>Count: {count}</p>
//!     </div>
//! );
//!
//! // Conditional rendering? Use the when! macro
//! let show = true;
//! let conditional = when!(show =>
//!     <p>Now you see me</p>
//! else
//!     <p>Now you don&apos;t</p>
//! );
//!
//! // Conditional classes? Easy!
//! let is_active = true;
//! let button = rsx!(
//!     <button class={if is_active { "active" } else { "" }}>
//!         Toggle
//!     </button>
//! );
//!
//! // Render lists with iterator magic
//! let items = vec!["A", "B", "C"];
//! let list = rsx!(
//!     <ul>
//!         {items.iter().map(|item| {
//!             let item = item.to_string();
//!             rsx!(<li>{item}</li>)
//!         })}
//!     </ul>
//! );
//! ```
//!
//! ## Components and Props - Build Reusable UI
//!
//! ```rust
//! extern crate alloc;
//! use momenta::prelude::*;
//!
//! // Define your props - just like React's PropTypes
//! #[derive(Default)]
//! struct ButtonProps {
//!     text: String,
//!     variant: String,
//!     children: Vec<Node>,
//! }
//!
//! // Create a component - clean and simple
//! #[component]
//! fn Button(props: &ButtonProps) -> Node {
//!     rsx!(
//!         <button class={format!("btn btn-{}", props.variant)}>
//!             {&props.text}
//!             {&props.children}
//!         </button>
//!     )
//! }
//!
//! // Use it anywhere!
//! let button = rsx!(
//!     <Button text="Click me" variant="primary">
//!         <span>"→"</span>
//!     </Button>
//! );
//! ```
//!
//! ## HTML Data attributes
//!
//! With Momenta, HTML data attributes are the only props which do not get validated by the compiler.
//! This allows you to use any valid literal or expression in the value of a data attribute.
//!
//! ```rust ignore
//! extern crate alloc;
//! use momenta::prelude::*;
//!
//! // Data attributes? No problem!
//! let element = rsx!(
//!     <div
//!         data_user="john"
//!         data_role="admin"
//!     />
//! );
//! ```
//!

extern crate alloc;

pub mod dom;
pub mod nodes;
pub mod signals;

pub mod prelude {
    #[cfg(feature = "wasm")]
    pub use crate::dom::render_root;
    pub use crate::nodes::{Component, Node};
    pub use crate::signals::{Signal, SignalValue, create_effect, create_signal};
    pub use momenta_macros::{component, rsx, when};
}
