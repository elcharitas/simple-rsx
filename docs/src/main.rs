#![no_std]
#![allow(unused_braces)]

extern crate alloc;
use alloc::{format, vec, vec::Vec};

use simple_rsx::Node;
use simple_rsx::component;
use simple_rsx::dom::render_root;
use simple_rsx::either;
use simple_rsx::rsx;
use simple_rsx::signals::create_effect;
use simple_rsx::signals::{Signal, SignalValue, create_signal};

#[derive(Clone, Copy, PartialEq)]
pub enum Page {
    // Start Here
    Home,
    GettingStarted,
    Tutorial,

    // Core Concepts
    Rsx,
    Signals,
    Effects,
    Resources,

    // Control Flow
    Show,
    For,

    // API Reference
    CreateSignal,
    CreateEffect,
    CreateMemo,
    CreateResource,

    // Guides
    Reactivity,
    Performance,
    Rust,
    Testing,
    Deployment,

    // Examples
    Counter,
    TodoMVC,
    HackerNews,
    RealWorld,
}

impl SignalValue for Page {
    fn as_any(&self) -> Option<&dyn core::any::Any> {
        Some(self)
    }
}

// Component Props
pub struct HeaderProps {
    pub current_page: Signal<Page>,
    pub theme: Signal<&'static str>,
    pub mobile_menu_open: Signal<bool>,
}

pub struct NavigationProps {
    pub current_page: Signal<Page>,
}

pub struct CodeBlockProps {
    pub code: &'static str,
    pub language: &'static str,
    pub filename: Option<&'static str>,
    pub highlight: Option<&'static str>,
}

pub struct TabsProps {
    pub tabs: Vec<(&'static str, &'static str)>,
    pub children: Vec<Node>,
}

pub struct PlaygroundProps {
    pub code: &'static str,
}

pub struct NoteProps {
    pub variant: &'static str,
    pub children: Vec<Node>,
}

// WASM bindings for highlight.js
#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = hljs)]
    pub fn highlightAll();
}

// Main App
#[component]
fn App() -> Node {
    let current_page = create_signal(Page::Home);
    let theme = create_signal("light");
    let mobile_menu_open = create_signal(false);

    create_effect(|| {
        highlightAll();
    });

    rsx! {
        <div class={format!("min-h-screen bg-white dark:bg-gray-950 {}", if theme.get() == "dark" { "dark" } else { "" })}>
            <Header
                current_page={current_page}
                theme={theme}
                mobile_menu_open={mobile_menu_open}
            />

            <div class="flex">
                // Sidebar Navigation
                {either!(current_page != Page::Home => <aside class="hidden lg:block w-64 shrink-0 border-r border-gray-200 dark:border-gray-800">
                        <div class="sticky top-14 h-[calc(100vh-3.5rem)] overflow-y-auto py-8">
                            <Navigation current_page={current_page} />
                        </div>
                    </aside>
                )}

                // Mobile Navigation
                {either!(mobile_menu_open.get() =>
                    <div class="lg:hidden fixed inset-0 z-50 flex">
                        <div class="fixed inset-0 bg-black/20 dark:bg-black/40" on_click={move |_| mobile_menu_open.set(false)}></div>
                        <div class="relative flex w-full max-w-xs flex-col bg-white dark:bg-gray-950">
                            <div class="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-800">
                                <span class="text-lg font-semibold">Navigation</span>
                                <button on_click={move |_| mobile_menu_open.set(false)} class="p-2">
                                    <i class="fas fa-times"></i>
                                </button>
                            </div>
                            <div class="overflow-y-auto p-4">
                                <Navigation current_page={current_page} />
                            </div>
                        </div>
                    </div>
                )}

                // Main Content
                <main class="flex-1 min-w-0">
                    {match current_page.get() {
                        Page::Home => rsx! { <HomePage /> },
                        Page::GettingStarted => rsx! { <GettingStartedPage /> },
                        Page::Tutorial => rsx! { <TutorialPage /> },
                        Page::Signals => rsx! { <SignalsPage /> },
                        Page::Effects => rsx! { <EffectsPage /> },
                        Page::Rsx => rsx! { <RsxPage /> },
                        Page::Resources => rsx! { <ResourcesPage /> },
                        Page::Show => rsx! { <ShowPage /> },
                        Page::For => rsx! { <ForPage /> },
                        Page::Reactivity => rsx! { <ReactivityPage /> },
                        Page::Counter => rsx! { <CounterExample /> },
                        _ => rsx! { <div class="p-8">"Page under construction..."</div> },
                    }}
                </main>

                // Right Sidebar (TOC)
                {either!(current_page!= Page::Home => <aside class="hidden xl:block w-64 shrink-0">
                        <div class="sticky top-14 h-[calc(100vh-3.5rem)] overflow-y-auto p-8">
                            // <TableOfContents current_page={current_page} />
                        </div>
                    </aside>
                )}
            </div>
        </div>
    }
}

// Header Component
#[component]
fn Header(props: &HeaderProps) -> Node {
    let current_page = props.current_page.clone();
    let theme = props.theme.clone();
    let mobile_menu_open = props.mobile_menu_open.clone();

    let toggle_theme = move |_| {
        theme.set(if theme.get() == "dark" {
            "light"
        } else {
            "dark"
        });
    };

    rsx! {
        <header class="sticky top-0 z-40 w-full border-b border-gray-200 dark:border-gray-800 bg-white/95 dark:bg-gray-950/95 backdrop-blur supports-[backdrop-filter]:bg-white/60 dark:supports-[backdrop-filter]:bg-gray-950/60">
            <div class="flex h-14 items-center px-4 sm:px-6 lg:px-8">
                <button
                    class="lg:hidden p-2 -ml-2"
                    on_click={move |_| mobile_menu_open.set(!mobile_menu_open.get())}
                >
                    <i class="fas fa-bars"></i>
                </button>

                <a href="#" on_click={move |_| current_page.set(Page::Home)} class="flex items-center space-x-2 ml-2 lg:ml-0">
                    <div class="w-8 h-8 bg-gradient-to-br from-blue-500 to-blue-600 rounded-lg flex items-center justify-center text-white">
                        <i class="fas fa-bolt"></i>
                    </div>
                    <span class="font-bold text-lg">Momenta</span>
                </a>

                <div class="ml-auto flex items-center space-x-4">
                    <nav class="hidden md:flex items-center space-x-6 mr-6">
                        <a href="#" on_click={move |_| current_page.set(Page::CreateSignal)}
                           class="text-sm font-medium transition-colors hover:text-blue-600 dark:hover:text-blue-400">
                            API
                        </a>
                        <a href="#" on_click={move |_| current_page.set(Page::GettingStarted)}
                           class="text-sm font-medium transition-colors hover:text-blue-600 dark:hover:text-blue-400">
                            Documentation
                        </a>
                        <a href="#"
                           class="text-sm font-medium transition-colors hover:text-blue-600 dark:hover:text-blue-400">
                            Playground
                        </a>
                    </nav>

                    <button
                        on_click={toggle_theme}
                        class="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
                    >
                        {either!(theme.get() == "dark" =>
                            <i class="fas fa-sun text-yellow-500"></i>
                        else
                            <i class="fas fa-moon text-gray-600"></i>
                        )}
                    </button>

                    <a href="https://github.com/momenta"
                       class="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors">
                        <i class="fab fa-github"></i>
                    </a>
                </div>
            </div>
        </header>
    }
}

// Navigation Component
#[component]
fn Navigation(props: &NavigationProps) -> Node {
    let current = props.current_page.clone();

    let nav_link = move |page: Page, label: &'static str| {
        let is_active = current.get() == page;
        let class = if is_active {
            "block px-3 py-1.5 text-sm font-medium text-blue-600 dark:text-blue-400"
        } else {
            "block px-3 py-1.5 text-sm text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200"
        };

        rsx! {
            <a href="#" on_click={move |_| current.set(page)} class={class}>
                {label}
            </a>
        }
    };

    let section = move |title: &'static str, children: Vec<Node>| {
        rsx! {
            <div class="mb-6">
                <h5 class="mb-2 px-3 text-xs font-semibold uppercase tracking-wider text-gray-900 dark:text-gray-100">
                    {title}
                </h5>
                <div class="space-y-1">
                    {children}
                </div>
            </div>
        }
    };

    rsx! {
        <nav class="px-2">
            {section("Start Here", vec![
                nav_link(Page::GettingStarted, "Getting Started"),
                nav_link(Page::Tutorial, "Tutorial"),
            ])}

            {section("Core Concepts", vec![
                nav_link(Page::Rsx, "rsx!"),
                nav_link(Page::Signals, "Signals"),
                nav_link(Page::Effects, "Effects"),
                nav_link(Page::Resources, "Resources"),
            ])}

            {section("Control Flow", vec![
                nav_link(Page::Show, "either!"),
                nav_link(Page::For, ".iter().map()"),
            ])}

            {section("API Reference", vec![
                nav_link(Page::CreateSignal, "create_signal"),
                nav_link(Page::CreateEffect, "create_effect"),
                nav_link(Page::CreateMemo, "create_memo"),
                nav_link(Page::CreateResource, "create_resource"),
            ])}

            {section("Guides", vec![
                nav_link(Page::Reactivity, "Reactivity"),
                nav_link(Page::Performance, "Performance"),
                nav_link(Page::Rust, "Rust"),
                nav_link(Page::Testing, "Testing"),
                nav_link(Page::Deployment, "Deployment"),
            ])}
        </nav>
    }
}

// Reusable Components
#[component]
fn CodeBlock(props: &CodeBlockProps) -> Node {
    rsx! {
        <div class="my-6 overflow-hidden rounded-lg border border-gray-200 dark:border-gray-800">
            {if let Some(filename) = props.filename {
                rsx! {
                    <div class="flex items-center justify-between border-b border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900 px-4 py-2">
                        <span class="text-xs font-medium text-gray-600 dark:text-gray-400">{filename}</span>
                        <button class="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200">
                            <i class="fas fa-copy text-xs"></i>
                        </button>
                    </div>
                }
            } else {
                rsx! { <div></div> }
            }}
            <div class="bg-gray-50 dark:bg-gray-900">
                <pre class="overflow-x-auto">
                    <code class={format!("language-{} text-sm", props.language)}>{props.code}</code>
                </pre>
            </div>
        </div>
    }
}

#[component]
fn Note(props: &NoteProps) -> Node {
    let (bg, border, icon) = match props.variant {
        "info" => (
            "bg-blue-50 dark:bg-blue-950/30",
            "border-blue-200 dark:border-blue-800",
            "fa-info-circle text-blue-600",
        ),
        "warning" => (
            "bg-amber-50 dark:bg-amber-950/30",
            "border-amber-200 dark:border-amber-800",
            "fa-exclamation-triangle text-amber-600",
        ),
        "tip" => (
            "bg-green-50 dark:bg-green-950/30",
            "border-green-200 dark:border-green-800",
            "fa-lightbulb text-green-600",
        ),
        _ => (
            "bg-gray-50 dark:bg-gray-900",
            "border-gray-200 dark:border-gray-800",
            "fa-info-circle text-gray-600",
        ),
    };

    rsx! {
        <div class={format!("my-6 rounded-lg border {} {} p-4", border, bg)}>
            <div class="flex">
                <i class={format!("fas {} mr-3 mt-0.5", icon)}></i>
                <div class="text-sm">
                    {&props.children}
                </div>
            </div>
        </div>
    }
}

#[component]
fn Playground(props: &PlaygroundProps) -> Node {
    rsx! {
        <div class="my-8 overflow-hidden rounded-lg border border-gray-200 dark:border-gray-800">
            <div class="grid md:grid-cols-2">
                <div class="border-r border-gray-200 dark:border-gray-800">
                    <div class="border-b border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900 px-4 py-2">
                        <span class="text-xs font-medium text-gray-600 dark:text-gray-400">Code</span>
                    </div>
                    <div class="bg-gray-50 dark:bg-gray-900">
                        <pre class="overflow-x-auto">
                            <code class="language-rust text-sm">{props.code}</code>
                        </pre>
                    </div>
                </div>
                <div>
                    <div class="border-b border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900 px-4 py-2">
                        <span class="text-xs font-medium text-gray-600 dark:text-gray-400">Output</span>
                    </div>
                    <div class="p-4">
                        <div class="text-sm text-gray-600 dark:text-gray-400">
                            [Interactive output would be displayed here]
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

// Page Components
#[component]
fn HomePage() -> Node {
    rsx! {
        <div class="mx-auto max-w-4xl px-4 py-16 sm:px-6 lg:px-8">
            <div class="text-center py-16">
                <h1 class="text-4xl font-bold tracking-tight text-gray-900 dark:text-gray-100 sm:text-5xl">
                    "Simple and performant reactivity for building user interfaces"
                </h1>
                <p class="mt-6 text-lg text-gray-600 dark:text-gray-400">
                    "Momenta makes it simple to build high-performance, reactive user interfaces using Rust's type system and ownership model."
                </p>
                <div class="mt-10 flex items-center justify-center gap-4">
                    <a href="#" class="rounded-lg bg-blue-600 px-6 py-3 text-sm font-semibold text-white hover:bg-blue-700">
                        "Get Started"
                    </a>
                    <a href="#" class="rounded-lg border border-gray-300 dark:border-gray-700 px-6 py-3 text-sm font-semibold hover:bg-gray-50 dark:hover:bg-gray-900">
                        "View on GitHub"
                    </a>
                </div>
            </div>

            <div class="mt-24 grid gap-8 sm:grid-cols-2 lg:grid-cols-3">
                <Feature
                    icon="fas fa-zap"
                    title="Fine-Grained Reactivity"
                    description="Automatically track dependencies and update only what has changed."
                />
                <Feature
                    icon="fas fa-code"
                    title="Familiar API"
                    description="Inspired by SolidJS with a Rust-first approach to reactive programming."
                />
                <Feature
                    icon="fas fa-shield-alt"
                    title="Type Safe"
                    description="Leverage Rust's type system for compile-time guarantees and better DX."
                />
                <Feature
                    icon="fas fa-feather"
                    title="Lightweight"
                    description="Small runtime with minimal overhead. Your apps stay fast."
                />
                <Feature
                    icon="fas fa-server"
                    title="SSR Ready"
                    description="Server-side rendering support out of the box for better performance."
                />
                <Feature
                    icon="fas fa-puzzle-piece"
                    title="Composable"
                    description="Build complex UIs from simple, reusable reactive primitives."
                />
            </div>

            <div class="mt-24">
                <h2 class="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-6">Quick Example</h2>
                <Playground code={r#"use momenta::prelude::*;

#[component]
fn Counter() -> Node {
    let count = create_signal(0);
    let doubled = create_memo(move || count * 2);
    
    rsx! {
        <div>
            <button on_click={move |_| count += 1}>
                Count: {count}
            </button>
            <p>Doubled: {doubled}</p>
        </div>
    }
}"#} />
            </div>
        </div>
    }
}

#[component]
fn Feature(props: &FeatureProps) -> Node {
    rsx! {
        <div class="rounded-lg border border-gray-200 dark:border-gray-800 p-6">
            <div class="mb-4 flex h-12 w-12 items-center justify-center rounded-lg bg-blue-100 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400">
                <i class={props.icon}></i>
            </div>
            <h3 class="mb-2 font-semibold text-gray-900 dark:text-gray-100">{props.title}</h3>
            <p class="text-sm text-gray-600 dark:text-gray-400">{props.description}</p>
        </div>
    }
}

pub struct FeatureProps {
    pub icon: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}

#[component]
fn SignalsPage() -> Node {
    rsx! {
        <article class="mx-auto max-w-4xl px-4 py-12 sm:px-6 lg:px-8">
            <header class="mb-12">
                <h1 class="text-4xl font-bold text-gray-900 dark:text-gray-100">Signals</h1>
                <p class="mt-4 text-lg text-gray-600 dark:text-gray-400">
                    "Signals are the most basic reactive primitive in Momenta. They contain values that change over time."
                </p>
            </header>

            <section class="prose prose-gray dark:prose-invert max-w-none">
                <h2 id="introduction">Introduction</h2>
                <p>
                    "When you create a signal, you get a getter and setter function. The getter tracks any scope it's called in,
                    and the setter triggers updates to any computations that depend on the signal's value."
                </p>

                <h2 id="basic-example">Basic Example</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::prelude::*;

#[component]
fn App() -> Node {
    // Create a signal with initial value 0
    let count = create_signal(0);
    
    rsx! {
        <div>
            <p>Count: {count}</p>
            <button on_click={move |_| count += 1}>
                "Increment"
            </button>
        </div>
    }
}"#}
                />

                <Note variant="info">
                    <p>
                        <strong>"Good to know:"</strong> " Unlike other frameworks, You  accessing a signal's value requires calling "
                        <code>".get()"</code> ". This explicit call enables Momenta's fine-grained reactivity system to track dependencies precisely."
                    </p>
                </Note>

                <h2 id="api-reference">API Reference</h2>

                <h3 id="creating-signals">Creating Signals</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Basic signal creation
let count = create_signal(0);
let name = create_signal("Alice".to_string());
let todos = create_signal(vec![]);

// With type annotations
let typed: Signal<i32> = create_signal(0);
let items: Signal<Vec<String>> = create_signal(vec![]);"#}
                />

                <h3 id="reading-values">Reading Values</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"let count = create_signal(5);

// Get current value
let value = count.get(); // 5

// Use in reactive context
create_effect(move || {
    println!("Count is: {}", count);
});

// Use with closures
let doubled = move || count * 2;"#}
                />

                <h3 id="updating-values">Updating Values</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"let count = create_signal(0);

// Set new value
count.set(5);

// Update based on previous value
count.update(|n| n + 1);

// Conditional updates
count.update(|n| if n < 10 { n + 1 } else { n });"#}
                />

                <h2>Advanced Patterns</h2>

                <h3>Derived Signals</h3>
                <p>"You can create derived values that automatically update when their dependencies change:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"let first = create_signal("John");
let last = create_signal("Doe");

// This will re-run whenever first or last changes
let full_name = create_memo(move || {
    format!("{} {}", first, last)
});

// Use in your component
rsx! {
    <p>Hello, {full_name}!</p>
}"#}
                />

                <Note variant="tip">
                    <p>
                        <strong>"Performance tip:"</strong> " Use " <code>"create_memo"</code> " for expensive computations.
                        Memos cache their results and only recompute when dependencies change."
                    </p>
                </Note>

                <h3>Signal Utilities</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Check if signals are equal
let a = create_signal(5);
let b = create_signal(5);
let are_equal = create_memo(move || a.get() == b.get());

// Combine multiple signals
let x = create_signal(2);
let y = create_signal(3);
let sum = create_memo(move || x.get() + y.get());

// Toggle boolean signals
let visible = create_signal(false);
let toggle = move |_| visible.update(|v| !v);"#}
                />

                <h2>Best Practices</h2>
                <ul>
                    <li>"Keep signals at the appropriate scope - not everything needs to be global state"</li>
                    <li>"Use " <code>"create_memo"</code> " for derived state instead of storing computed values"</li>
                    <li>"Prefer " <code>".update()"</code> " over " <code>".get()"</code> " + " <code>".set()"</code> " for transformations"</li>
                    <li>"Group related signals into structs for better organization"</li>
                </ul>

                <div class="mt-12 flex items-center justify-between border-t border-gray-200 dark:border-gray-800 pt-6">
                    <a href="#" class="text-sm text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200">
                        "← Getting Started"
                    </a>
                    <a href="#" class="text-sm text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200">
                        "Effects →"
                    </a>
                </div>
            </section>
        </article>
    }
}

#[component]
pub fn EffectsPage() -> Node {
    rsx! {
        <article class="mx-auto max-w-4xl px-4 py-12 sm:px-6 lg:px-8">
            <header class="mb-12">
                <h1 class="text-4xl font-bold text-gray-900 dark:text-gray-100">Effects</h1>
                <p class="mt-4 text-lg text-gray-600 dark:text-gray-400">
                    "Effects are the building blocks of reactivity in Momenta. They run code in response to changes in signals."
                </p>
            </header>
            <section class="prose prose-gray dark:prose-invert max-w-none">
                <h2 id="introduction">Introduction</h2>
                <p>
                    "Effects are functions that run when their dependencies change. They are the building blocks of reactivity in Momenta."
                </p>
                <h2 id="basic-example">Basic Example</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::prelude::*;"#}
                />
                <h2 id="api-reference">API Reference</h2>
                <h3 id="creating-effects">Creating Effects</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Basic effect creation"#}
                />
                <h3 id="cleaning-up-effects">Cleaning Up Effects</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Clean up effects when they are no longer needed"#}
                />
                <h2>Advanced Patterns</h2>
                <h3>Effect Cleanup</h3>
                <p>"Effects can return a cleanup function that is called when the effect is no longer needed:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Effect with cleanup"#}
                />
                <h3>Effect Dependencies</h3>
                <p>"Effects can depend on multiple signals, and they will only run when any of their dependencies change:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Effect with multiple dependencies"#}
                />
                <h3>Effect Ordering</h3>
                <p>"Effects are executed in the order they are created, and their cleanup functions are executed in reverse order:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Effect ordering"#}
                />
                <h2>Best Practices</h2>
                <ul>
                    <li>"Keep effects as lightweight as possible"</li>
                    <li>"Avoid creating effects inside loops or other complex logic"</li>
                    <li>"Use cleanup functions to avoid memory leaks"</li>
                    <li>"Use " <code>"create_effect"</code> " for simple effects"</li>
                </ul>
                <div class="mt-12 flex items-center justify-between border-t border-gray-200 dark:border-gray-800 pt-6">
                    <a href="#" class="text-sm text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200">
                        "← Signals"
                    </a>
                    <a href="#" class="text-sm text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200">
                        "Resources →"
                    </a>
                </div>
            </section>
        </article>
    }
}

// Add more page implementations...
#[component]
fn GettingStartedPage() -> Node {
    rsx! {
        <article class="mx-auto max-w-4xl px-4 py-12 sm:px-6 lg:px-8">
            <header class="mb-12">
                <h1 class="text-4xl font-bold text-gray-900 dark:text-gray-100">Getting Started</h1>
                <p class="mt-4 text-lg text-gray-600 dark:text-gray-400">
                    "Get up and running with Momenta in minutes."
                </p>
            </header>

            <section class="prose prose-gray dark:prose-invert max-w-none">
                <h2>Installation</h2>
                <p>"Add Momenta to your " <code>"Cargo.toml"</code> ":"</p>
                <CodeBlock
                    filename="Cargo.toml"
                    highlight=""
                    language="toml"
                    code={r#"[dependencies]
momenta = "0.1"

# For web projects
[dependencies.web-sys]
version = "0.3"
features = ["Document", "Element", "HtmlElement"]"#}
                />

                <h2>Your First Component</h2>
                <CodeBlock
                    filename="src/main.rs"
                    highlight=""
                    language="rust"
                    code={r#"use momenta::prelude::*;

#[component]
fn App() -> Node {
    let name = create_signal("World");
    
    rsx! {
        <div class="container">
            <h1>"Hello, {name.get()}!"</h1>
            <input 
                type="text"
                value={name.get()}
                on_input={move |e| name.set(e.value())}
                placeholder="Enter your name"
            />
        </div>
    }
}

fn main() {
    mount_to_body(App);
}"#}
                />

                <h2>Project Structure</h2>
                <p>"A typical Momenta project structure looks like this:"</p>
                <CodeBlock
                    language="text"
                    highlight=""
                    filename=""
                    code={r#"my-app/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── components/
│   │   ├── mod.rs
│   │   ├── header.rs
│   │   └── footer.rs
│   └── pages/
│       ├── mod.rs
│       └── home.rs
├── static/
│   └── index.html
└── style/
    └── main.css"#}
                />

                <Note variant="tip">
                    <p>
                        <strong>"VS Code Extension:"</strong> " Install the Momenta VS Code extension for syntax highlighting,
                        code completion, and inline error checking."
                    </p>
                </Note>
            </section>
        </article>
    }
}

// Stub implementations
#[component]
fn TutorialPage() -> Node {
    rsx! { <div class="p-8">"Tutorial page..."</div> }
}

#[component]
fn RsxPage() -> Node {
    rsx! { <div class="p-8">"RSX page..."</div> }
}
#[component]
fn ResourcesPage() -> Node {
    rsx! { <div class="p-8">"Resources page..."</div> }
}
#[component]
fn ContextPage() -> Node {
    rsx! { <div class="p-8">"Context page..."</div> }
}
#[component]
fn StoresPage() -> Node {
    rsx! { <div class="p-8">"Stores page..."</div> }
}
#[component]
fn ShowPage() -> Node {
    rsx! { <div class="p-8">"Show page..."</div> }
}
#[component]
fn ForPage() -> Node {
    rsx! { <div class="p-8">"For page..."</div> }
}
#[component]
fn ReactivityPage() -> Node {
    rsx! { <div class="p-8">"Reactivity page..."</div> }
}
#[component]
fn CounterExample() -> Node {
    rsx! { <div class="p-8">"Counter example..."</div> }
}

fn main() {
    render_root::<App>("app");
}
