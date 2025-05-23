# Simple RSX

A lightweight and intuitive JSX-like syntax implementation for Rust, enabling HTML-like templating with the full power of Rust's type system.

[![Crates.io](https://img.shields.io/crates/v/simple-rsx.svg)](https://crates.io/crates/simple-rsx)
[![Documentation](https://docs.rs/simple-rsx/badge.svg)](https://docs.rs/simple-rsx)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Coverage Status](https://coveralls.io/repos/github/elcharitas/simple-rsx/badge.svg?branch=main)](https://coveralls.io/github/elcharitas/simple-rsx?branch=main)

## Features

- [x] JSX-like syntax support in Rust
- [x] Self-closing tags (`<div />`)
- [x] Element attributes
- [x] Nested elements
- [x] String literals as children
- [x] Fragment support (`<></>`)
- [x] Basic error handling and validation
- [x] Tag matching validation
- [x] Procedural macro implementation
- [x] Basic HTML element rendering
- [x] Expression support in braces (`{expr}`)
- [x] attribute value concatenation and interpolation
- [x] Conditional attributes
- [x] Conditional rendering
- [x] Looping
- [x] List rendering
- [x] Custom components
- [x] Props validation
- [x] Props binding
- [ ] State management
- [ ] Lifecycle hooks
- [ ] Event handling
- [x] Server-side rendering

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
simple-rsx = "0.1"
```

## Why Simple RSX?

I started this project while attempting to transit my [portfolio](https://elcharitas.wtf) from Next.js to Rust. I tried using dioxus, yew, and hypertext, but I found them to be too complex and verbose for my needs. I wanted a simple and intuitive way to write HTML-like templates in Rust, while still leveraging the full power of Rust's type system.

Simple RSX aims to provide a straightforward and intuitive way to write HTML-like templates in Rust, while still leveraging the full power of Rust's type system. By using Rust's syntax and features, Simple RSX enables you to write expressive and type-safe code while still maintaining the simplicity and readability of HTML.

## Usage

```rust
use simple_rsx::*;

// Self-closing tag with attributes
let element = rsx!(<div class="container" id="app" />);

// Nested elements with text content
let nested = rsx!(
    <div class="container" id="app">
        <p>Hello, world!</p>
    </div>
);

// element with attributes and children
let element_with_children = rsx!(
    <div class="container" id="app">
        <p>Hello, world!</p>
        <p>Another paragraph.</p>
    </div>
);

// Fragment syntax
let fragment = rsx!(
    <>
        <h1>Title</h1>
        <p>Content</p>
    </>
);

// Expression support
let count = 42;
let expression = rsx!(<p>Count: {count}</p>);

// Conditional rendering
let show = true;
let conditional = rsx!(
    <div>
        {either!(show => <p>This is shown.</p>) else <p>This is hidden.</p>}
    </div>
);

// List rendering
let items = &["Item 1", "Item 2", "Item 3"];
let list = rsx!(
    <ul>
        {items.iter().map(|item| rsx!(<li>{item}</li>))}
    </ul>
);

// Server-side rendering
let nodes = rsx!(
    <html>
        <head>
            <title>"My Page"</title>
        </head>
        <body>
            <h1>"Welcome to my page!"</h1>
        </body>
    </html>
);
let html = nodes.to_string();
```

## Project Structure

- `simple-rsx`: Main library crate
- `simple-rsx-macros`: Procedural macros implementation

## Development Status

This project is currently in active development. While basic JSX syntax is supported, many advanced features are still in progress. Contributions are welcome!

## Alternatives

- [Yew](https://crates.io/crates/yew)
- [Hypertext](https://crates.io/crates/hypertext)

## License

MIT License
