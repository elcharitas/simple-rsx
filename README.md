# Simple RSX

A lightweight and intuitive JSX-like syntax implementation for Rust, enabling HTML-like templating with the full power of Rust's type system.

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
- [ ] Expression support in braces (`{expr}`)
- [ ] Props validation
- [ ] Event handling
- [ ] Conditional rendering
- [ ] Looping
- [ ] List rendering
- [ ] Custom components
- [ ] State management
- [ ] Lifecycle hooks
- [x] Server-side rendering
- [ ] Server-side rendering with hydration

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
simple-rsx = "0.1.0"
```

## Usage

```rust
use simple_rsx::*;

// Self-closing tag with attributes
let element = jsx!(<div class="container" id="app" />);

// Nested elements with text content
let nested = jsx!(
    <div class="container" id="app">
        <p>"Hello, world!"</p>
    </div>
);

// Fragment syntax
let fragment = jsx!(
    <>
        <h1>"Title"</h1>
        <p>"Content"</p>
    </>
);
```

## Project Structure

- `simple-rsx`: Main library crate
- `simple-rsx-macros`: Procedural macros implementation

## Development Status

This project is currently in active development. While basic JSX syntax is supported, many advanced features are still in progress. Contributions are welcome!

## License

MIT License
