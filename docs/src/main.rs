#![no_std]
#![allow(unused_braces)]

extern crate alloc;
use alloc::{format, vec};

use simple_rsx::Node;
use simple_rsx::component;
use simple_rsx::dom::render_root;
use simple_rsx::either;
use simple_rsx::rsx;
use simple_rsx::signals::SignalValue;
use simple_rsx::signals::create_signal;

#[derive(Clone, Copy, PartialEq)]
enum Page {
    Landing,
    Installation,
    GetStarted,
    Counter,
    Concepts,
}

impl SignalValue for Page {
    fn as_any(&self) -> Option<&dyn core::any::Any> {
        Some(self)
    }
}

fn navbar_class(is_active: bool) -> &'static str {
    either!(is_active => "navbar-item has-text-grey-dark {}" else "navbar-item has-text-weight-semibold")
}

#[component]
fn App() -> Node {
    let current_page = create_signal(Page::Landing);
    let nav = move |page: Page| {
        move |_| {
            current_page.set(page);
        }
    };

    rsx! {
        <div class="has-background-white">
            // Modern header inspired by TailwindCSS docs
            <header class="navbar is-white is-fixed-top" style="border-bottom: 1px solid #e5e7eb; backdrop-filter: blur(8px);">
                <div class="container is-fluid">
                    <div class="navbar-brand">
                        <div class="navbar-item">
                            <a class="is-flex is-align-items-center" on_click={nav(Page::Landing)}>
                                <div class="icon is-medium mr-3" style="color: #06B6D4;">
                                    <i class="fas fa-code fa-lg"></i>
                                </div>
                                <div>
                                    <h1 class="title is-5 mb-0 has-text-grey-darker">Simple RSX</h1>
                                </div>
                            </a>
                        </div>
                    </div>
                    <div class="navbar-menu">
                        <div class="navbar-start">
                            <a class={navbar_class(current_page.get() == Page::Installation)}
                               href="#" on_click={nav(Page::Installation)} style="font-size: 0.9rem;">
                                Installation
                            </a>
                            <a class={navbar_class(current_page.get() == Page::GetStarted)}
                               href="#" on_click={nav(Page::GetStarted)} style="font-size: 0.9rem;">
                                Get Started
                            </a>
                            <a class={navbar_class(current_page.get() == Page::Counter)}
                               href="#" on_click={nav(Page::Counter)} style="font-size: 0.9rem;">
                                Examples
                            </a>
                            <a class={navbar_class(current_page.get() == Page::Concepts)}
                               href="#" on_click={nav(Page::Concepts)} style="font-size: 0.9rem;">
                                Concepts
                            </a>
                        </div>
                        <div class="navbar-end">
                            <div class="navbar-item">
                                <a class="button is-small is-outlined" href="https://github.com/elcharitas/simple-rsx" target="_blank" style="border-color: #d1d5db; color: #6b7280;">
                                    <span class="icon is-small"><i class="fab fa-github"></i></span>
                                    <span>GitHub</span>
                                </a>
                            </div>
                        </div>
                    </div>
                </div>
            </header>

            // Main content area
            <div style="padding-top: 3.25rem; min-height: 100vh;" class="has-background-white">
                {
                    match current_page.get() {
                        Page::Landing => rsx! { <LandingPage /> },
                        Page::Installation => rsx! { <InstallationPage /> },
                        Page::GetStarted => rsx! { <GetStartedPage /> },
                        Page::Counter => rsx! { <CounterAppPage /> },
                        Page::Concepts => rsx! { <div /> },
                    }
                }
            </div>

            // Minimal footer
            <footer class="section py-5 has-background-grey-lighter" style="border-top: 1px solid #e5e7eb;">
                <div class="container">
                    <div class="has-text-centered">
                        <p class="has-text-grey is-size-7">
                            Built with Simple RSX Open source on <a href="https://github.com/elcharitas/simple-rsx" class="has-text-link">GitHub</a>
                        </p>
                    </div>
                </div>
            </footer>
        </div>
    }
}

#[component]
fn LandingPage() -> Node {
    rsx! {
        <div>
            // Hero section - TailwindCSS docs style
            <section class="section py-6 has-background-white">
                <div class="container">
                    <div class="columns is-centered">
                        <div class="column is-8 has-text-centered">
                            <div class="mb-6">
                                <div class="icon is-large mb-4" style="color: #06B6D4;">
                                    <i class="fas fa-code fa-3x"></i>
                                </div>
                                <h1 class="title is-1 has-text-grey-darker mb-4" style="font-weight: 800; letter-spacing: -0.025em;">
                                    Simple RSX
                                </h1>
                                <p class="subtitle is-4 has-text-grey-dark mb-6" style="font-weight: 400; line-height: 1.5;">
                                    A React-inspired JSX Library for Rust
                                </p>
                                <p class="is-size-5 has-text-grey mb-6" style="line-height: 1.75; max-width: 42rem; margin: 0 auto;">
                                    Build modern, reactive user interfaces in Rust with familiar JSX syntax, reactive signals, and zero-cost abstractions.
                                </p>
                                <div class="buttons is-centered">
                                    <button class="button is-medium mr-3" style="background-color: #06B6D4; color: white; border: 0; font-weight: 600; padding: 0.75rem 1.5rem;">
                                        Get Started
                                    </button>
                                    <button class="button is-medium is-outlined" style="border-color: #d1d5db; color: #6b7280; font-weight: 600; padding: 0.75rem 1.5rem;">
                                        View on GitHub
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Features section
            <section class="section py-6" style="background-color: #f9fafb;">
                <div class="container">
                    <div class="has-text-centered mb-6">
                        <h2 class="title is-2 has-text-grey-darker mb-4" style="font-weight: 700;">
                            Why Simple RSX?
                        </h2>
                        <p class="subtitle is-5 has-text-grey-dark" style="font-weight: 400;">
                            Experience the power of Rust with the familiarity of React
                        </p>
                    </div>
                    <div class="columns is-multiline">
                        <div class="column is-4">
                            <div class="box has-background-white p-6" style="border: 1px solid #e5e7eb; box-shadow: none; border-radius: 0.75rem;">
                                <div class="has-text-centered mb-4">
                                    <div class="icon is-large mb-3" style="color: #06B6D4;">
                                        <i class="fas fa-bolt fa-2x"></i>
                                    </div>
                                    <h3 class="title is-4 has-text-grey-darker mb-3" style="font-weight: 600;">Lightning Fast</h3>
                                    <p class="has-text-grey-dark" style="line-height: 1.6; font-size: 0.95rem;">
                                        Zero-cost abstractions with compile-time optimizations. No runtime overhead, just pure Rust performance.
                                    </p>
                                </div>
                            </div>
                        </div>
                        <div class="column is-4">
                            <div class="box has-background-white p-6" style="border: 1px solid #e5e7eb; box-shadow: none; border-radius: 0.75rem;">
                                <div class="has-text-centered mb-4">
                                    <div class="icon is-large mb-3" style="color: #06B6D4;">
                                        <i class="fas fa-code fa-2x"></i>
                                    </div>
                                    <h3 class="title is-4 has-text-grey-darker mb-3" style="font-weight: 600;">Familiar Syntax</h3>
                                    <p class="has-text-grey-dark" style="line-height: 1.6; font-size: 0.95rem;">
                                        JSX-like syntax that feels natural to React developers, with the power and safety of Rusts type system.
                                    </p>
                                </div>
                            </div>
                        </div>
                        <div class="column is-4">
                            <div class="box has-background-white p-6" style="border: 1px solid #e5e7eb; box-shadow: none; border-radius: 0.75rem;">
                                <div class="has-text-centered mb-4">
                                    <div class="icon is-large mb-3" style="color: #06B6D4;">
                                        <i class="fas fa-shield-alt fa-2x"></i>
                                    </div>
                                    <h3 class="title is-4 has-text-grey-darker mb-3" style="font-weight: 600;">Memory Safe</h3>
                                    <p class="has-text-grey-dark" style="line-height: 1.6; font-size: 0.95rem;">
                                        Rusts ownership system ensures memory safety without garbage collection or runtime checks.
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}

#[component]
fn InstallationPage() -> Node {
    rsx! {
        <section class="section py-6 has-background-white">
            <div class="container">
                <div class="columns">
                    <div class="column is-8 is-offset-2">
                        <div class="mb-6">
                            <h1 class="title is-1 has-text-grey-darker mb-4" style="font-weight: 800;">
                                Installation
                            </h1>
                            <p class="subtitle is-5 has-text-grey-dark" style="font-weight: 400; line-height: 1.6;">
                                Get Simple RSX up and running in your project
                            </p>
                        </div>

                        <div class="content">
                            <div class="mb-6">
                                <h2 class="title is-3 has-text-grey-darker mb-4" style="font-weight: 700;">
                                    Prerequisites
                                </h2>
                                <p class="has-text-grey-dark mb-4" style="line-height: 1.6;">
                                    Make sure you have Rust installed on your system. If not, install it from <a href="https://rustup.rs/" target="_blank" class="has-text-link">rustup.rs</a>.
                                </p>
                                <div class="notification is-light" style="background-color: #eff6ff; border: 1px solid #bfdbfe; border-left: 4px solid #3b82f6;">
                                    <p class="has-text-grey-darker">
                                        <span class="icon has-text-link"><i class="fas fa-info-circle"></i></span>
                                        <strong>Minimum Rust version:</strong> 1.70+
                                    </p>
                                </div>
                            </div>

                            <div class="mb-6">
                                <h2 class="title is-3 has-text-grey-darker mb-4" style="font-weight: 700;">
                                    Add to Your Project
                                </h2>
                                <p class="has-text-grey-dark mb-4" style="line-height: 1.6;">
                                    Add Simple RSX to your <code class="px-2 py-1" style="background-color: #f3f4f6; border-radius: 0.25rem; color: #1f2937;">Cargo.toml</code>:
                                </p>

                                <div style="border: 1px solid #e5e7eb; border-radius: 0.5rem; overflow: hidden;" class="mb-4">
                                    <div style="background-color: #1e293b; padding: 1rem;">
                                        <pre style="background: transparent; color: #e2e8f0; font-size: 0.9rem; line-height: 1.5;">
                                            <div style="color: #f472b6; display: block">[dependencies]</div>
                                            <span style="color: #34d399;">simple-rsx</span> = <span style="color: #fbbf24;">"0.1.0"</span>
                                        </pre>
                                    </div>
                                </div>

                                <p class="has-text-grey-dark mb-3" style="line-height: 1.6;">Or use cargo add:</p>
                                <div style="border: 1px solid #e5e7eb; border-radius: 0.5rem; overflow: hidden;">
                                    <div style="background-color: #1e293b; padding: 1rem;">
                                        <pre style="background: transparent; color: #e2e8f0; font-size: 0.9rem;">
                                            <span style="color: #34d399;">$ cargo add simple-rsx</span>
                                        </pre>
                                    </div>
                                </div>
                            </div>

                            <div class="mb-6">
                                <h2 class="title is-3 has-text-grey-darker mb-4" style="font-weight: 700;">
                                    Feature Flags
                                </h2>
                                <p class="has-text-grey-dark mb-4" style="line-height: 1.6;">
                                    Simple RSX supports several optional features:
                                </p>

                                <div class="table-container">
                                    <table class="table is-fullwidth" style="border: 1px solid #e5e7eb; border-radius: 0.5rem;">
                                        <thead style="background-color: #f9fafb;">
                                            <tr>
                                                <th class="has-text-grey-darker" style="font-weight: 600; border-bottom: 1px solid #e5e7eb;">Feature</th>
                                                <th class="has-text-grey-darker" style="font-weight: 600; border-bottom: 1px solid #e5e7eb;">Description</th>
                                                <th class="has-text-grey-darker" style="font-weight: 600; border-bottom: 1px solid #e5e7eb;">Default</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            <tr style="border-bottom: 1px solid #f3f4f6;">
                                                <td><code class="px-2 py-1" style="background-color: #f3f4f6; border-radius: 0.25rem; color: #06B6D4;">wasm</code></td>
                                                <td class="has-text-grey-dark">Client rendering support</td>
                                                <td><span class="tag is-small" style="background-color: #fee2e2; color: #dc2626;">No</span></td>
                                            </tr>
                                            <tr style="border-bottom: 1px solid #f3f4f6;">
                                                <td><code class="px-2 py-1" style="background-color: #f3f4f6; border-radius: 0.25rem; color: #06B6D4;">router</code></td>
                                                <td class="has-text-grey-dark">Client-side routing utilities</td>
                                                <td><span class="tag is-small" style="background-color: #fee2e2; color: #dc2626;">No</span></td>
                                            </tr>
                                            <tr>
                                                <td><code class="px-2 py-1" style="background-color: #f3f4f6; border-radius: 0.25rem; color: #06B6D4;">async</code></td>
                                                <td class="has-text-grey-dark">Async component support</td>
                                                <td><span class="tag is-small" style="background-color: #fee2e2; color: #dc2626;">No</span></td>
                                            </tr>
                                        </tbody>
                                    </table>
                                </div>
                            </div>

                            <div>
                                <h2 class="title is-3 has-text-grey-darker mb-4" style="font-weight: 700;">
                                    Verify Installation
                                </h2>
                                <p class="has-text-grey-dark mb-4" style="line-height: 1.6;">
                                    Create a simple test to verify everything works:
                                </p>

                                <div style="border: 1px solid #e5e7eb; border-radius: 0.5rem; overflow: hidden;" class="mb-4">
                                    <div style="background-color: #1e293b; padding: 1rem;">
                                        <pre style="background: transparent; color: #e2e8f0; font-size: 0.9rem; line-height: 1.5;">
                                            <code>
                                            // TODO: verification code would go here
                                            </code>
                                        </pre>
                                    </div>
                                </div>

                                <div class="notification is-light" style="background-color: #f0fdf4; border: 1px solid #bbf7d0; border-left: 4px solid #22c55e;">
                                    <div class="is-flex is-align-items-center">
                                        <span class="icon has-text-success mr-3"><i class="fas fa-check-circle"></i></span>
                                        <div>
                                            <strong class="has-text-grey-darker">Success!</strong>
                                            <span class="has-text-grey-dark">Youre ready to start building with Simple RSX.</span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn GetStartedPage() -> Node {
    rsx! {
        <section class="section py-6 has-background-white">
            <div class="container">
                <div class="columns">
                    <div class="column is-8 is-offset-2">
                        <div class="mb-6">
                            <h1 class="title is-1 has-text-grey-darker mb-4" style="font-weight: 800;">
                                Get Started
                            </h1>
                            <p class="subtitle is-5 has-text-grey-dark" style="font-weight: 400; line-height: 1.6;">
                                Learn Simple RSX fundamentals step by step
                            </p>
                        </div>

                        <div class="content">
                            <div class="mb-6" style="border: 1px solid #e5e7eb; border-radius: 0.75rem; overflow: hidden;">
                                <div class="px-5 py-4" style="background-color: #f9fafb; border-bottom: 1px solid #e5e7eb;">
                                    <h2 class="title is-4 has-text-grey-darker mb-0" style="font-weight: 600;">
                                        <span class="icon has-text-link mr-2"><i class="fas fa-puzzle-piece"></i></span>
                                        Your First Component
                                    </h2>
                                </div>
                                <div class="px-5 py-4">
                                    <p class="has-text-grey-dark mb-4" style="line-height: 1.6;">
                                        Lets create your first Simple RSX component. Components are the building blocks of your application.
                                    </p>
                                    <div style="border: 1px solid #e5e7eb; border-radius: 0.5rem; overflow: hidden;">
                                        <div style="background-color: #1e293b; padding: 1rem;">
                                            <pre style="background: transparent; color: #e2e8f0; font-size: 0.9rem; line-height: 1.5;">
                                                <code>
                                                    // TODO: component code would go here
                                                </code>
                                            </pre>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            <div class="mb-6" style="border: 1px solid #e5e7eb; border-radius: 0.75rem; overflow: hidden;">
                                <div class="px-5 py-4" style="background-color: #f9fafb; border-bottom: 1px solid #e5e7eb;">
                                    <h2 class="title is-4 has-text-grey-darker mb-0" style="font-weight: 600;">
                                        <span class="icon mr-2" style="color: #06B6D4;"><i class="fas fa-broadcast-tower"></i></span>
                                        Using Signals for State
                                    </h2>
                                </div>
                                <div class="px-5 py-4">
                                    <p class="has-text-grey-dark mb-4" style="line-height: 1.6;">
                                        Signals provide reactive state management. When a signal changes, components automatically re-render.
                                    </p>
                                    <div style="border: 1px solid #e5e7eb; border-radius: 0.5rem; overflow: hidden;" class="mb-4">
                                        <div style="background-color: #1e293b; padding: 1rem;">
                                            <pre style="background: transparent; color: #e2e8f0; font-size: 0.9rem; line-height: 1.5;">
                                                <code>
                                                    // TODO: signals code would go here
                                                </code>
                                            </pre>
                                        </div>
                                    </div>
                                    <div class="notification is-light" style="background-color: #eff6ff; border: 1px solid #bfdbfe; border-left: 4px solid #3b82f6;">
                                        <p class="has-text-grey-darker">
                                            <span class="icon has-text-link"><i class="fas fa-lightbulb"></i></span>
                                            <strong>Pro Tip:</strong> Signals automatically track dependencies and only re-render affected components.
                                        </p>
                                    </div>
                                </div>
                            </div>

                            <div style="border: 1px solid #e5e7eb; border-radius: 0.75rem; overflow: hidden;">
                                <div class="px-5 py-4" style="background-color: #f9fafb; border-bottom: 1px solid #e5e7eb;">
                                    <h2 class="title is-4 has-text-grey-darker mb-0" style="font-weight: 600;">
                                        <span class="icon mr-2" style="color: #10b981;"><i class="fas fa-mouse-pointer"></i></span>
                                        Event Handling
                                    </h2>
                                </div>
                                <div class="px-5 py-4">
                                    <p class="has-text-grey-dark mb-4" style="line-height: 1.6;">
                                        Handle user interactions with event handlers. Simple RSX supports all standard DOM events.
                                    </p>

                                    <div class="columns is-multiline">
                                        <div class="column is-4">
                                            <div class="box p-4" style="background-color: #fef3c7; border: 1px solid #fde68a;">
                                                <h4 class="title is-6 has-text-grey-darker mb-3" style="font-weight: 600;">
                                                    <span class="icon"><i class="fas fa-mouse"></i></span>
                                                    <span>Mouse Events</span>
                                                </h4>
                                                <div class="tags">
                                                    <span class="tag is-small" style="background-color: #fbbf24; color: white;"><code>on_click</code></span>
                                                    <span class="tag is-small" style="background-color: #fbbf24; color: white;"><code>on_mouseenter</code></span>
                                                    <span class="tag is-small" style="background-color: #fbbf24; color: white;"><code>on_mouseleave</code></span>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="column is-4">
                                            <div class="box p-4" style="background-color: #dbeafe; border: 1px solid #bfdbfe;">
                                                <h4 class="title is-6 has-text-grey-darker mb-3" style="font-weight: 600;">
                                                    <span class="icon"><i class="fas fa-keyboard"></i></span>
                                                    <span>Keyboard Events</span>
                                                </h4>
                                                <div class="tags">
                                                    <span class="tag is-small" style="background-color: #3b82f6; color: white;"><code>on_keydown</code></span>
                                                    <span class="tag is-small" style="background-color: #3b82f6; color: white;"><code>on_keyup</code></span>
                                                    <span class="tag is-small" style="background-color: #3b82f6; color: white;"><code>on_keypress</code></span>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="column is-4">
                                            <div class="box p-4" style="background-color: #d1fae5; border: 1px solid #a7f3d0;">
                                                <h4 class="title is-6 has-text-grey-darker mb-3" style="font-weight: 600;">
                                                    <span class="icon"><i class="fas fa-edit"></i></span>
                                                    <span>Form Events</span>
                                                </h4>
                                                <div class="tags">
                                                    <span class="tag is-small" style="background-color: #10b981; color: white;"><code>on_input</code></span>
                                                    <span class="tag is-small" style="background-color: #10b981; color: white;"><code>on_change</code></span>
                                                    <span class="tag is-small" style="background-color: #10b981; color: white;"><code>on_submit</code></span>
                                                </div>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="notification is-light" style="background-color: #f0fdf4; border: 1px solid #bbf7d0; border-left: 4px solid #22c55e;">
                                        <p class="has-text-grey-darker">
                                            <span class="icon has-text-success"><i class="fas fa-info-circle"></i></span>
                                            <strong>Pro Tip:</strong> Use closures to capture variables from the surrounding scope in your event handlers.
                                        </p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn CounterAppPage() -> Node {
    let counter_example = r#""#;
    let advanced_example = r#""#;

    rsx! {
        <section class="section py-6 has-background-white">
            <div class="container">
                <div class="columns">
                    <div class="column is-8 is-offset-2">
                        <div class="mb-6">
                            <h1 class="title is-1 has-text-grey-darker mb-4" style="font-weight: 800;">
                                Examples
                            </h1>
                            <p class="subtitle is-5 has-text-grey-dark" style="font-weight: 400; line-height: 1.6;">
                                Learn by example with practical code snippets
                            </p>
                        </div>

                        <div class="content">
                            <div class="mb-6" style="border: 1px solid #e5e7eb; border-radius: 0.75rem; overflow: hidden;">
                                <div class="px-5 py-4" style="background-color: #06B6D4; color: white;">
                                    <h2 class="title is-4 has-text-white mb-0" style="font-weight: 600;">
                                        <span class="icon has-text-white mr-2"><i class="fas fa-play"></i></span>
                                        Basic Counter
                                    </h2>
                                </div>
                                <div class="px-5 py-4">
                                    <p class="has-text-grey-dark mb-4" style="line-height: 1.6;">
                                        A simple counter demonstrating signals and event handling:
                                    </p>

                                    <div style="border: 1px solid #e5e7eb; border-radius: 0.5rem; overflow: hidden;" class="mb-4">
                                        <div style="background-color: #1e293b; padding: 1rem;">
                                            <pre style="background: transparent; color: #e2e8f0; font-size: 0.85rem; line-height: 1.5;">
                                                <code>{counter_example}</code>
                                            </pre>
                                        </div>
                                    </div>

                                    <div class="notification is-light" style="background-color: #eff6ff; border: 1px solid #bfdbfe; border-left: 4px solid #3b82f6;">
                                        <p class="has-text-grey-darker">
                                            <span class="icon has-text-link"><i class="fas fa-lightbulb"></i></span>
                                            <strong>Key Features:</strong> Signal creation, event handlers, reactive updates
                                        </p>
                                    </div>
                                </div>
                            </div>

                            <div class="mb-6" style="border: 1px solid #e5e7eb; border-radius: 0.75rem; overflow: hidden;">
                                <div class="px-5 py-4" style="background-color: #3b82f6; color: white;">
                                    <h2 class="title is-4 has-text-white mb-0" style="font-weight: 600;">
                                        <span class="icon has-text-white mr-2"><i class="fas fa-cogs"></i></span>
                                        Advanced Counter
                                    </h2>
                                </div>
                                <div class="px-5 py-4">
                                    <p class="has-text-grey-dark mb-4" style="line-height: 1.6;">
                                        A more complex counter with configurable step size and history tracking:
                                    </p>

                                    <div style="border: 1px solid #e5e7eb; border-radius: 0.5rem; overflow: hidden;">
                                        <div style="background-color: #1e293b; padding: 1rem;">
                                            <pre style="background: transparent; color: #e2e8f0; font-size: 0.8rem; line-height: 1.5;">
                                                <code>{advanced_example}</code>
                                            </pre>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            <div style="border: 1px solid #e5e7eb; border-radius: 0.75rem; overflow: hidden;">
                                <div class="px-5 py-4" style="background-color: #10b981; color: white;">
                                    <h3 class="title is-4 has-text-white mb-0" style="font-weight: 600;">
                                        <span class="icon has-text-white mr-2"><i class="fas fa-graduation-cap"></i></span>
                                        Key Concepts Demonstrated
                                    </h3>
                                </div>
                                <div class="px-5 py-4">
                                    <div class="columns is-multiline">
                                        <div class="column is-6">
                                            <div class="is-flex is-align-items-start mb-4">
                                                <span class="icon has-text-link mr-3 mt-1">
                                                    <i class="fas fa-broadcast-tower"></i>
                                                </span>
                                                <div>
                                                    <p class="has-text-weight-semibold has-text-grey-darker">Signal creation and updates</p>
                                                    <p class="is-size-7 has-text-grey-dark">Reactive state management with automatic re-rendering</p>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="column is-6">
                                            <div class="is-flex is-align-items-start mb-4">
                                                <span class="icon has-text-link mr-3 mt-1">
                                                    <i class="fas fa-mouse-pointer"></i>
                                                </span>
                                                <div>
                                                    <p class="has-text-weight-semibold has-text-grey-darker">Event handler closures</p>
                                                    <p class="is-size-7 has-text-grey-dark">Capturing variables in event callbacks</p>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="column is-6">
                                            <div class="is-flex is-align-items-start mb-4">
                                                <span class="icon has-text-warning mr-3 mt-1">
                                                    <i class="fas fa-eye"></i>
                                                </span>
                                                <div>
                                                    <p class="has-text-weight-semibold has-text-grey-darker">Conditional rendering</p>
                                                    <p class="is-size-7 has-text-grey-dark">Dynamic UI based on state conditions</p>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="column is-6">
                                            <div class="is-flex is-align-items-start mb-4">
                                                <span class="icon has-text-danger mr-3 mt-1">
                                                    <i class="fas fa-palette"></i>
                                                </span>
                                                <div>
                                                    <p class="has-text-weight-semibold has-text-grey-darker">Dynamic class names</p>
                                                    <p class="is-size-7 has-text-grey-dark">Styling based on component state</p>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="column is-12">
                                            <div class="is-flex is-align-items-start">
                                                <span class="icon has-text-link mr-3 mt-1">
                                                    <i class="fas fa-history"></i>
                                                </span>
                                                <div>
                                                    <p class="has-text-weight-semibold has-text-grey-darker">State history management</p>
                                                    <p class="is-size-7 has-text-grey-dark">Tracking and displaying state changes over time</p>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn ConceptsPage() -> Node {
    rsx! {
        <section class="section py-6 has-background-white">
            <div class="container">
                <div class="columns">
                    <div class="column is-8 is-offset-2">
                        <div class="mb-6">
                            <h1 class="title is-1 has-text-grey-darker mb-4" style="font-weight: 800;">
                                Core Concepts
                            </h1>
                            <p class="subtitle is-5 has-text-grey-dark" style="font-weight: 400; line-height: 1.6;">
                                Master the fundamental building blocks of Simple RSX
                            </p>
                        </div>

                        <div class="content">
                            <div class="columns is-multiline">
                                <div class="column is-6">
                                    <div class="box p-5" style="border: 1px solid #e5e7eb; border-radius: 0.75rem; height: 100%;">
                                        <div class="mb-4">
                                            <div class="is-flex is-align-items-center mb-3">
                                                <span class="icon is-medium mr-3" style="color: #06B6D4;">
                                                    <i class="fas fa-broadcast-tower fa-lg"></i>
                                                </span>
                                                <h3 class="title is-4 has-text-grey-darker mb-0" style="font-weight: 600;">Signals</h3>
                                            </div>
                                            <p class="has-text-grey-dark mb-4" style="line-height: 1.6;">
                                                Reactive state management through signals. Components automatically re-render when signals they depend on change.
                                            </p>
                                        </div>

                                        <div>
                                            <h4 class="has-text-weight-semibold has-text-grey-darker mb-3">
                                                <span class="icon has-text-link"><i class="fas fa-star"></i></span>
                                                Benefits:
                                            </h4>
                                            <div class="content">
                                                <ul class="has-text-grey-dark" style="font-size: 0.9rem; line-height: 1.5;">
                                                    <li><strong>Fine-grained reactivity:</strong> Only affected components re-render</li>
                                                    <li><strong>Automatic dependency tracking:</strong> No manual subscriptions needed</li>
                                                    <li><strong>Minimal re-renders:</strong> Optimized performance by default</li>
                                                    <li><strong>Thread-safe by default:</strong> Built for concurrent environments</li>
                                                </ul>
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                <div class="column is-6">
                                    <div class="box p-5" style="border: 1px solid #e5e7eb; border-radius: 0.75rem; height: 100%;">
                                        <div class="mb-4">
                                            <div class="is-flex is-align-items-center mb-3">
                                                <span class="icon is-medium mr-3" style="color: #3b82f6;">
                                                    <i class="fas fa-magic fa-lg"></i>
                                                </span>
                                                <h3 class="title is-4 has-text-grey-darker mb-0" style="font-weight: 600;">Effects</h3>
                                            </div>
                                            <p class="has-text-grey-dark mb-4" style="line-height: 1.6;">
                                                Side effects and subscriptions that run in response to signal changes. Perfect for API calls, timers, and cleanup logic.
                                            </p>
                                        </div>

                                        <div>
                                            <h4 class="has-text-weight-semibold has-text-grey-darker mb-3">
                                                <span class="icon has-text-info"><i class="fas fa-wrench"></i></span>
                                                Use Cases:
                                            </h4>
                                            <div class="content">
                                                <ul class="has-text-grey-dark" style="font-size: 0.9rem; line-height: 1.5;">
                                                    <li><strong>Data fetching:</strong> Automatic API calls on state changes</li>
                                                    <li><strong>DOM manipulation:</strong> Direct DOM updates when needed</li>
                                                    <li><strong>Event subscriptions:</strong> WebSocket and external event handling</li>
                                                    <li><strong>Resource cleanup:</strong> Automatic cleanup on component unmount</li>
                                                </ul>
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                <div class="column is-6">
                                    <div class="box p-5" style="border: 1px solid #e5e7eb; border-radius: 0.75rem; height: 100%;">
                                        <div class="mb-4">
                                            <div class="is-flex is-align-items-center mb-3">
                                                <span class="icon is-medium mr-3" style="color: #10b981;">
                                                    <i class="fas fa-layer-group fa-lg"></i>
                                                </span>
                                                <h3 class="title is-4 has-text-grey-darker mb-0" style="font-weight: 600;">Rendering Scopes</h3>
                                            </div>
                                            <p class="has-text-grey-dark mb-4" style="line-height: 1.6;">
                                                Isolated rendering contexts that manage component lifecycles and cleanup automatically. Each scope owns its signals and effects.
                                            </p>
                                        </div>

                                        <div>
                                            <h4 class="has-text-weight-semibold has-text-grey-darker mb-3">
                                                <span class="icon has-text-success"><i class="fas fa-shield-alt"></i></span>
                                                Features:
                                            </h4>
                                            <div class="content">
                                                <ul class="has-text-grey-dark" style="font-size: 0.9rem; line-height: 1.5;">
                                                    <li><strong>Automatic cleanup:</strong> Memory leaks are prevented</li>
                                                    <li><strong>Memory safety:</strong> Rusts ownership ensures correctness</li>
                                                    <li><strong>Hierarchical structure:</strong> Parent-child scope relationships</li>
                                                    <li><strong>Performance isolation:</strong> Scoped optimization boundaries</li>
                                                </ul>
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                <div class="column is-6">
                                    <div class="box p-5" style="border: 1px solid #e5e7eb; border-radius: 0.75rem; height: 100%;">
                                        <div class="mb-4">
                                            <div class="is-flex is-align-items-center mb-3">
                                                <span class="icon is-medium mr-3" style="color: #f59e0b;">
                                                    <i class="fas fa-puzzle-piece fa-lg"></i>
                                                </span>
                                                <h3 class="title is-4 has-text-grey-darker mb-0" style="font-weight: 600;">Components and Props</h3>
                                            </div>
                                            <p class="has-text-grey-dark mb-4" style="line-height: 1.6;">
                                                Reusable UI components with type-safe props. Components are just Rust functions that return renderable content.
                                            </p>
                                        </div>

                                        <div>
                                            <h4 class="has-text-weight-semibold has-text-grey-darker mb-3">
                                                <span class="icon has-text-warning"><i class="fas fa-check-double"></i></span>
                                                Advantages:
                                            </h4>
                                            <div class="content">
                                                <ul class="has-text-grey-dark" style="font-size: 0.9rem; line-height: 1.5;">
                                                    <li><strong>Type safety:</strong> Compile-time prop validation</li>
                                                    <li><strong>Compile-time validation:</strong> Catch errors before runtime</li>
                                                    <li><strong>Zero-cost abstractions:</strong> No runtime overhead</li>
                                                    <li><strong>Easy composition:</strong> Mix and match components freely</li>
                                                </ul>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            <div class="box p-6 mt-6" style="border: 1px solid #e5e7eb; border-radius: 0.75rem; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white;">
                                <h3 class="title is-3 has-text-white mb-5" style="font-weight: 700;">
                                    <span class="icon-text">
                                        <span class="icon has-text-white"><i class="fas fa-graduation-cap"></i></span>
                                        <span>Learning Path</span>
                                    </span>
                                </h3>
                                <div class="columns">
                                    <div class="column">
                                        <div class="box has-background-white p-5" style="border-radius: 0.75rem; height: 100%;">
                                            <div class="has-text-centered mb-3">
                                                <span class="icon is-large" style="color: #06B6D4;">
                                                    <i class="fas fa-play-circle fa-2x"></i>
                                                </span>
                                            </div>
                                            <h4 class="title is-5 has-text-grey-darker has-text-centered mb-3" style="font-weight: 600;">1. Start with Components</h4>
                                            <p class="has-text-grey-dark has-text-centered" style="font-size: 0.9rem; line-height: 1.5;">
                                                Learn to create basic components using the <code class="px-2 py-1" style="background-color: #f3f4f6; border-radius: 0.25rem;">rsx!</code> macro.
                                            </p>
                                        </div>
                                    </div>
                                    <div class="column">
                                        <div class="box has-background-white p-5" style="border-radius: 0.75rem; height: 100%;">
                                            <div class="has-text-centered mb-3">
                                                <span class="icon is-large" style="color: #3b82f6;">
                                                    <i class="fas fa-mouse-pointer fa-2x"></i>
                                                </span>
                                            </div>
                                            <h4 class="title is-5 has-text-grey-darker has-text-centered mb-3" style="font-weight: 600;">2. Add Interactivity</h4>
                                            <p class="has-text-grey-dark has-text-centered" style="font-size: 0.9rem; line-height: 1.5;">
                                                Use signals to manage state and event handlers for user input.
                                            </p>
                                        </div>
                                    </div>
                                    <div class="column">
                                        <div class="box has-background-white p-5" style="border-radius: 0.75rem; height: 100%;">
                                            <div class="has-text-centered mb-3">
                                                <span class="icon is-large" style="color: #10b981;">
                                                    <i class="fas fa-magic fa-2x"></i>
                                                </span>
                                            </div>
                                            <h4 class="title is-5 has-text-grey-darker has-text-centered mb-3" style="font-weight: 600;">3. Master Effects</h4>
                                            <p class="has-text-grey-dark has-text-centered" style="font-size: 0.9rem; line-height: 1.5;">
                                                Handle side effects and cleanup with the effect system.
                                            </p>
                                        </div>
                                    </div>
                                </div>
                                <div class="has-text-centered mt-5">
                                    <button class="button is-white is-medium" style="font-weight: 600; padding: 0.75rem 1.5rem;">
                                        <span class="icon"><i class="fas fa-book-open"></i></span>
                                        <span>Start Learning</span>
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

fn main() {
    render_root::<App>("app");
}
