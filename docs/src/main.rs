#![no_std]

extern crate alloc;
use alloc::{format, vec, vec::Vec};
use momenta::prelude::*;

static GITHUB_LINK: &str = "https://github.com/elcharitas/momenta";
static CRATES_LINK: &str = "https://crates.io/crates/momenta";

#[derive(Clone, Copy, PartialEq, SignalValue)]
pub enum Page {
    Home,

    // Start Here
    GettingStarted,
    Philosophy,

    // Core Concepts
    Rsx,
    Signals,
    Effects,
    Resources,
    Components,

    // Control Flow
    When,
    Lists,

    // Guides
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
        <div class={format!("min-h-screen bg-white dark:bg-gray-950 {}", if theme == "dark" { "dark" } else { "" })}>
            <Header {current_page} {theme} {mobile_menu_open} />

            <div class="flex">
                // Sidebar Navigation
                {when!(current_page != Page::Home => <aside class="hidden lg:block w-64 shrink-0 border-r border-gray-200 dark:border-gray-800">
                        <div class="sticky top-14 h-[calc(100vh-3.5rem)] overflow-y-auto py-8">
                            <Navigation {current_page} />
                        </div>
                    </aside>
                )}

                // Mobile Navigation
                {when!(mobile_menu_open =>
                    <div class="lg:hidden fixed inset-0 z-50 flex">
                        <div class="fixed inset-0 bg-black/20 dark:bg-black/40" on:click={move |_| mobile_menu_open.set(false)}></div>
                        <div class="relative flex w-full max-w-xs flex-col bg-white dark:bg-gray-950">
                            <div class="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-800">
                                <span class="text-lg font-semibold">Navigation</span>
                                <button type="button" on:click={move |_| mobile_menu_open.set(false)} class="p-2">
                                    <i class="fas fa-times"></i>
                                </button>
                            </div>
                            <div class="overflow-y-auto p-4">
                                <Navigation {current_page} />
                            </div>
                        </div>
                    </div>
                )}

                // Main Content
                <main class="flex-1 min-w-0">
                    {when!(current_page.get() {
                        Page::Home => <HomePage {current_page} />,
                        Page::GettingStarted => <GettingStartedPage />,
                        Page::Philosophy => <PhilosophyPage />,
                        Page::Signals => <SignalsPage />,
                        Page::Effects => <EffectsPage />,
                        Page::Rsx => <RsxPage />,
                        Page::Resources => <ResourcesPage />,
                        Page::When => <ShowPage />,
                        Page::Lists => <ForPage />,
                        Page::Components => <ComponentsPage />,
                        Page::Counter => <CounterExample />,
                        _ => <div class="p-8">"Page under construction..."</div>,
                    })}
                </main>

                // Right Sidebar (TOC)
                {when!(current_page != Page::Home => <aside class="hidden xl:block w-64 shrink-0">
                        <div class="sticky top-14 h-[calc(100vh-3.5rem)] overflow-y-auto p-8">
                            // <TableOfContents {current_page} />
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
    let current_page = props.current_page;
    let theme = props.theme;
    let mobile_menu_open = props.mobile_menu_open;

    let toggle_theme = move |_| {
        theme.set(if theme == "dark" { "light" } else { "dark" });
    };

    rsx! {
        <header class={"sticky top-0 z-40 w-full border-b border-gray-200 dark:border-gray-800 bg-white/95 dark:bg-gray-950/95 backdrop-blur supports-[backdrop-filter]:bg-white/60 dark:supports-[backdrop-filter]:bg-gray-950/60"}>
            <div class="flex h-14 items-center px-4 sm:px-6 lg:px-8">
                <button
                    class="lg:hidden p-2 -ml-2"
                    on:click={move |_| mobile_menu_open.set(!mobile_menu_open)}
                >
                    <i class="fas fa-bars"></i>
                </button>

                <a href="#" on:click={move |_| current_page.set(Page::Home)} class="flex items-center space-x-2 ml-2 lg:ml-0">
                    <img src="./static/icon.svg" alt="Momenta Logo" class="w-8 h-8" />
                    <span class="font-bold text-lg">Momenta</span>
                </a>

                <div class="ml-auto flex items-center space-x-4">
                    <nav class="hidden md:flex items-center space-x-6 mr-6">
                        <a href="#" on:click={move |_| current_page.set(Page::Performance)}
                           class="text-sm font-medium transition-colors hover:text-blue-600 dark:hover:text-blue-400">
                            Guides
                        </a>
                        <a href="#" on:click={move |_| current_page.set(Page::GettingStarted)}
                           class="text-sm font-medium transition-colors hover:text-blue-600 dark:hover:text-blue-400">
                            Documentation
                        </a>
                        <a href="#"
                           class="text-sm font-medium transition-colors hover:text-blue-600 dark:hover:text-blue-400">
                            Playground
                        </a>
                    </nav>

                    <button
                        on:click={toggle_theme}
                        class="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
                    >
                        {when!(theme == "dark" =>
                            <i class="fas fa-sun text-yellow-500"></i>
                        else
                            <i class="fas fa-moon text-gray-600"></i>
                        )}
                    </button>

                    <a href={GITHUB_LINK}
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
    let current = props.current_page;

    let nav_link = move |page: Page, label: &'static str| {
        let is_active = current == page;
        let class = if is_active {
            "block px-3 py-1.5 text-sm font-medium text-blue-600 dark:text-blue-400"
        } else {
            "block px-3 py-1.5 text-sm text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200"
        };

        rsx! {
            <a href="#" on:click={move |_| current.set(page)} class={class}>
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
                nav_link(Page::Philosophy, "Philosophy"),
            ])}

            {section("Reactive Primitives", vec![
                nav_link(Page::Rsx, "rsx!"),
                nav_link(Page::Components, "#[component]"),
                nav_link(Page::Signals, "create_signal"),
                nav_link(Page::Effects, "create_effect"),
                nav_link(Page::Resources, "create_resource"),
            ])}

            {section("Control Flow", vec![
                nav_link(Page::When, "when!"),
                nav_link(Page::Lists, ".iter().map()"),
            ])}

            {section("Guides", vec![
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
            {when!(let Some(filename) = props.filename =>
                <div class="flex items-center justify-between border-b border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900 px-4 py-2">
                    <span class="text-xs font-medium text-gray-600 dark:text-gray-400">{filename}</span>
                    <button class="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200">
                        <i class="fas fa-copy text-xs">"😉"</i>
                    </button>
                </div>
            )}
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
            <div class="flex flex-col md:flex-row items-stretch h-full">
                <div class="w-1/2 border-r border-gray-200 dark:border-gray-800 flex flex-col">
                    <div class="border-b border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900 px-4 py-2">
                        <span class="text-xs font-medium text-gray-600 dark:text-gray-400">Code</span>
                    </div>
                    <div class="bg-gray-50 dark:bg-gray-900 flex-1">
                        <pre class="overflow-x-auto h-full">
                            <code class="language-rust text-xs overflow-x">{props.code}</code>
                        </pre>
                    </div>
                </div>
                <div class="w-1/2 flex flex-col">
                    <div class="border-b border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900 px-4 py-2">
                        <span class="text-xs font-medium text-gray-600 dark:text-gray-400">Output</span>
                    </div>
                    <div class="flex-1 text-sm text-gray-600 dark:text-gray-400">
                        <CounterExample />
                    </div>
                </div>
            </div>
        </div>
    }
}

// Page Components
#[component]
fn HomePage(props: &NavigationProps) -> Node {
    let current_page = props.current_page;
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
                    <a href="#" on:click={move |_| current_page.set(Page::GettingStarted)} class="rounded-lg bg-blue-600 px-6 py-3 text-sm font-semibold text-white hover:bg-blue-700">
                        "Get Started"
                    </a>
                    <a href={GITHUB_LINK} class="rounded-lg border border-gray-300 dark:border-gray-700 px-6 py-3 text-sm font-semibold hover:bg-gray-50 dark:hover:bg-gray-900">
                        "View on GitHub"
                    </a>
                    <a href={CRATES_LINK} class="rounded-lg bg-yellow-600 border border-gray-300 dark:border-gray-700 px-6 py-3 text-sm font-semibold hover:bg-yellow-700 dark:hover:bg-yellow-900">
                        "View on Crates.io"
                    </a>
                </div>
            </div>

            <div class="mt-24 grid gap-8 sm:grid-cols-2 lg:grid-cols-3">
                <Feature
                    icon="fas fa-zap"
                    title="Element-Level Reactivity"
                    description="Automatically track dependencies and update only what has changed."
                />
                <Feature
                    icon="fas fa-code"
                    title="Familiar API"
                    description="Inspired by React with a Rust-first approach to reactive programming."
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
                <Playground
                    code={r#"use momenta::prelude::*;

#[component]
fn CounterExample() -> Node {
    let mut count = create_signal(0);
    rsx! {
        <div class="bg-gradient-to-br from-purple-400 to-blue-600 flex items-center justify-center p-4">
            <div class="bg-white/20 backdrop-blur-lg rounded-3xl p-8 shadow-2xl border border-white/30">
                <h1 class="text-3xl font-bold text-white mb-6 text-center">
                    "Momenta Counter"
                </h1>
                <div class="text-6xl font-bold text-center mb-8 transition-all duration-300 text-white">
                    {count}
                </div>
                <div class="flex gap-4 justify-center">
                    <button
                        class="px-6 py-3 bg-red-500 hover:bg-red-600 text-white font-semibold rounded-xl transition-all duration-200 transform hover:scale-105 shadow-lg"
                        on:click={move |_| count -= 1}
                    >
                        "− Decrease"
                    </button>
                    <button
                        class="px-6 py-3 bg-green-500 hover:bg-green-600 text-white font-semibold rounded-xl transition-all duration-200 transform hover:scale-105 shadow-lg"
                        on:click={move |_| count += 1}
                    >
                        "+ Increase"
                    </button>
                </div>
                <button
                    class="w-full mt-4 px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg transition-colors"
                    on:click={move |_| count.set(0)}
                >
                    "Reset Count: " {count}
                </button>
            </div>
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
fn ForPage() -> Node {
    rsx! {
        <article class="mx-auto max-w-4xl px-4 py-12 sm:px-6 lg:px-8">
            <header class="mb-12">
                <h1 class="text-4xl font-bold text-gray-900 dark:text-gray-100">List Rendering</h1>
                <p class="mt-4 text-lg text-gray-600 dark:text-gray-400">
                    "Rendering lists of items efficiently using Rust's iterators."
                </p>
            </header>
            <section class="prose prose-gray dark:prose-invert max-w-none">
                <h2 id="introduction">Introduction</h2>
                <p>
                    "List rendering is a common task in UI development. Momenta leverages Rust's powerful iterator system to provide efficient and type-safe list rendering."
                </p>

                <h2 id="basic-example">Basic Example</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::prelude::*;

#[component]
fn FruitList() -> Node {
    let mut fruits = create_signal(vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Cherry".to_string(),
    ]);
    
    rsx! {
        <div>
            <h2 class="font-bold uppercase">"Fruit List"</h2>
            <ul>
                {fruits.map(|fruit| rsx! {
                    <li>{fruit}</li>
                })}
            </ul>
            <button on:click={move |_| {
                fruits.push("Orange".to_string());
            }}>
                "Add Orange"
            </button>
        </div>
    }
}"#}
                />

                <Note variant="info">
                    <p>
                        <strong>"Good to know:"</strong> " When rendering lists, Momenta efficiently updates only the items that have changed, added, or removed."
                    </p>
                </Note>

                <h2 id="syntax">Basic Syntax</h2>
                <p>"The basic pattern for rendering lists in Momenta uses Rust's iterator methods:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Basic list rendering pattern
let items = vec!["Item 1", "Item 2", "Item 3"];

rsx! {
    <ul>
        {items.map(|item| rsx! {
            <li>{item}</li>
        })}
    </ul>
}

// With a signal
let items = create_signal(vec!["Item 1", "Item 2", "Item 3"]);

rsx! {
    <ul>
        {items.map(|item| rsx! {
            <li>{item}</li>
        })}
    </ul>
}"#}
                />

                <h2 id="advanced-patterns">Advanced Patterns</h2>

                <h3 id="with-indices">Working with Indices</h3>
                <p>"Sometimes you need the index of each item in the list:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Using enumerate() to get indices
let items = create_signal(vec!["Apple", "Banana", "Cherry"]);

rsx! {
    <ul>
        {items.enumerate().map(|(index, item)| rsx! {
            <li>"Item #" {index + 1} ": " {item}</li>
        })}
    </ul>
}"#}
                />

                <h3 id="complex-items">Complex Item Types</h3>
                <p>"You can render lists of complex types:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Define a struct for list items
struct Todo {
    id: usize,
    text: String,
    completed: bool,
}

#[component]
fn TodoList() -> Node {
    let todos = create_signal(vec![
        Todo { id: 1, text: "Learn Momenta".to_string(), completed: false },
        Todo { id: 2, text: "Build an app".to_string(), completed: false },
        Todo { id: 3, text: "Share with friends".to_string(), completed: true },
    ]);
    
    let new_todo_text = create_signal(String::new());
    
    let add_todo = move |_| {
        let text = new_todo_text.get();
        if !text.is_empty() {
            let next_id = todos.map(|todo| todo.id).max().unwrap_or(0) + 1;
            todos.push(Todo {
                id: next_id,
                completed: false,
                text,
            });
            new_todo_text.set(String::new());
        }
    };
    
    let toggle_todo = move |id: usize| {
        if let Some(todo) = todos.iter_mut().find(|todo| todo.id == id) {
            todo.completed = !todo.completed;
        }
    };
    
    rsx! {
        <div>
            <h2 class="font-bold uppercase">"Todo List"</h2>
            <ul class="space-y-2">
                {todos.map(|todo| {
                    let id = todo.id;
                    rsx! {
                        <li class={format!("flex items-center {}", 
                            if todo.completed { "line-through text-gray-400" } else { "" }
                        )}>
                            <input 
                                type="checkbox" 
                                checked={todo.completed} 
                                on:change={move |_| toggle_todo(id)}
                                class="mr-2"
                            />
                            {todo.text}
                        </li>
                    }
                })}
            </ul>
            
            <div class="mt-4 flex">
                <input 
                    type="text" 
                    value={new_todo_text}
                    on:input={move |e| new_todo_text.set(e.value())}
                    placeholder="Add a new todo"
                    class="border p-2 rounded-l"
                />
                <button 
                    on:click={add_todo}
                    class="bg-blue-500 text-white p-2 rounded-r"
                >
                    "Add"
                </button>
            </div>
        </div>
    }
}"#}
                />

                <h3 id="filtering-sorting">Filtering and Sorting</h3>
                <p>"You can use Rust's iterator methods to filter and sort items before rendering:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Filtering and sorting example
let numbers = create_signal(vec![5, 2, 8, 1, 9, 3, 7, 4, 6]);
let show_even_only = create_signal(false);
let sort_ascending = create_signal(true);

rsx! {
    <div>
        <div class="mb-4">
            <label class="mr-4">
                <input 
                    type="checkbox" 
                    checked={show_even_only}
                    on:change={move |_| show_even_only.set(!show_even_only.get())}
                    class="mr-2"
                />
                "Show even numbers only"
            </label>
            
            <label>
                <input 
                    type="checkbox" 
                    checked={sort_ascending}
                    on:change={move |_| sort_ascending.set(!sort_ascending.get())}
                    class="mr-2"
                />
                "Sort ascending"
            </label>
        </div>
        
        <ul class="grid grid-cols-3 gap-2">
            {{
                let mut filtered = numbers.get();
                
                // Apply filtering if needed
                if show_even_only.get() {
                    filtered.retain(|n| n % 2 == 0);
                }
                
                // Apply sorting
                if sort_ascending.get() {
                    filtered.sort();
                } else {
                    filtered.sort_by(|a, b| b.cmp(a));
                }
                
                // Map to nodes
                filtered.iter().map(|n| rsx! {
                    <li class="bg-gray-100 dark:bg-gray-800 p-2 rounded text-center">
                        {n}
                    </li>
                })
            }}
        </ul>
    </div>
}"#}
                />

                <h2 id="performance">Performance Considerations</h2>
                <p>"When rendering lists, keep these performance considerations in mind:"</p>
                <ul>
                    <li>"Use keys for list items when possible to help with efficient updates"</li>
                    <li>"Avoid recreating the entire list when only a few items change"</li>
                    <li>"For large lists, consider pagination or virtualization"</li>
                    <li>"Minimize the amount of work done in the map function"</li>
                    <li>"Pre-compute derived values outside of the render function"</li>
                </ul>

                <h2 class="font-bold uppercase">Best Practices</h2>
                <ul>
                    <li>"Use .iter().map() pattern for list rendering"</li>
                    <li>"Extract complex item rendering into separate components"</li>
                    <li>"Use signals for list data that changes over time"</li>
                    <li>"Leverage Rust's powerful iterator methods for filtering, sorting, and transforming data"</li>
                    <li>"Consider memoizing expensive computations for list items"</li>
                </ul>

                <div class="mt-12 flex items-center justify-between border-t border-gray-200 dark:border-gray-800 pt-6">
                    <a href="#" class="text-sm text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200">
                        "← Conditional Rendering"
                    </a>
                    <a href="#" class="text-sm text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200">
                        "Reactivity →"
                    </a>
                </div>
            </section>
        </article>
    }
}

#[component]
fn ResourcesPage() -> Node {
    rsx! {
        <article class="mx-auto max-w-4xl px-4 py-12 sm:px-6 lg:px-8">
            <header class="mb-12">
                <h1 title="" class="text-4xl font-bold text-gray-900 dark:text-gray-100">Resources</h1>
                <p class="mt-4 text-lg text-gray-600 dark:text-gray-400">
                    "Resources handle asynchronous data fetching with built-in loading and error states."
                </p>
            </header>

            <section class="prose prose-gray dark:prose-invert max-w-none">
                <h2 class="font-bold uppercase">Introduction</h2>
                <p>
                    "Resources are reactive primitives for handling asynchronous operations like API calls.
                    They automatically track loading states and provide a simple way to work with async data."
                </p>

                <h2 class="font-bold uppercase">Basic Example</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::prelude::*;

async fn fetch_user_data() -> Result<User, Error> {
    // Simulate API call
    let response = api_client.get("/user/profile").await?;
    Ok(response.json().await?)
}

#[component]
fn UserProfile() -> Node {
    // Create a resource that fetches user data
    let user = create_resource(fetch_user_data);
    
    rsx! {
        <div>
            {match user.get() {
                Some(Ok(user_data)) => rsx! {
                    <div>
                        <h1>{user_data.name}</h1>
                        <p>{user_data.email}</p>
                    </div>
                },
                Some(Err(error)) => rsx! {
                    <div class="error">Error: {error.to_string()}</div>
                },
                None => rsx! {
                    <div class="loading">Loading...</div>
                }
            }}
        </div>
    }
}"#}
                />

                <Note variant="info">
                    <p>
                        <strong>"Result support:"</strong> " Resources work with any type, including Result types for proper error handling.
                        The resource stores whatever your async function returns - None while loading, Some(value) when complete."
                    </p>
                </Note>

                <h2 class="font-bold uppercase">API Reference</h2>

                <h3>Creating Resources</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Basic resource - just pass an async function
let data = create_resource(async || { fetch_data().await });

// Resource with closure capturing variables
let user_id = create_signal(1);
let user = create_resource(move || async move {
    fetch_user(user_id.get()).await
});

// Resource that depends on multiple signals
let search_query = create_signal("".to_string());
let page = create_signal(1);
let results = create_resource(move || async move {
    let query = search_query.get();
    let current_page = page.get();
    search_posts(query, current_page).await
});"#}
                />

                <h3>Resource States</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Resource returning Result type
let resource = create_resource(async || { fetch_data().await });

// Handle all possible states
match resource.get() {
    None => {
        // Still loading initial data
        rsx! { <div>"Loading..."</div> }
    },
    Some(Ok(data)) => {
        // Data loaded successfully
        rsx! { <div>{data}</div> }
    },
    Some(Err(error)) => {
        // Error occurred during fetch
        rsx! { <div class="error">"Error: {error}"</div> }
    }
}

// Check detailed resource status
match resource.status().get() {
    ResourceStatus::Idle => { /* Not started yet */ },
    ResourceStatus::Pending => { /* Waiting to start */ },
    ResourceStatus::Loading => { /* Currently fetching */ },
    ResourceStatus::Resolved => { /* Data is available (success or error) */ },
}"#}
                />

                <h3>Retrying Resources</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"let data = create_resource(async || { fetch_data().await });

// Manually retry the resource
data.retry();

// Retry when a button is clicked
rsx! {
    <div>
        <button on:click={move |_| data.retry()}>
            "Retry"
        </button>
        {match data.get() {
            Some(Ok(value)) => rsx! { <div>{value}</div> },
            Some(Err(error)) => rsx! { <div class="error">Error: {error}</div> },
            None => rsx! { <div>"Loading..."</div> }
        }}
    </div>
}"#}
                />

                <h2 class="font-bold uppercase">Advanced Patterns</h2>

                <h3>Reactive Dependencies</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Resources automatically re-run when signals they depend on change
let user_id = create_signal(1);
let user_data = create_resource(move || async move {
    fetch_user(user_id.get()).await
});

// When user_id changes, the resource will automatically retry
let change_user = move |new_id| {
    user_id.set(new_id); // This triggers the resource to re-fetch
};"#}
                />

                <h3>Combining with Effects</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"let data = create_resource(async || { fetch_data().await });
let processed_data = create_signal(None);

// Process data when it becomes available
create_effect(move || {
    if let Some(Ok(raw_data)) = data.get() {
        let processed = process_data(raw_data);
        processed_data.set(Some(processed));
    }
});"#}
                />

                <h2 class="font-bold uppercase">Best Practices</h2>
                <ul>
                    <li>"Use resources for any asynchronous data fetching"</li>
                    <li>"Handle both loading (None) and loaded (Some) states"</li>
                    <li>"Return Result types from async functions for proper error handling"</li>
                    <li>"Use signals within resource closures to create reactive dependencies"</li>
                    <li>"Call retry() to manually re-fetch data when needed"</li>
                    <li>"Check resource.status() for detailed loading state information"</li>
                    <li>"Keep async functions simple and focused"</li>
                </ul>
            </section>
        </article>
    }
}

#[component]
fn PhilosophyPage() -> Node {
    rsx! {
        <article class="mx-auto max-w-4xl px-4 py-12 sm:px-6 lg:px-8">
            <header class="mb-12">
                <h1 class="text-4xl font-bold text-gray-900 dark:text-gray-100">Philosophy</h1>
                <p class="mt-4 text-lg text-gray-600 dark:text-gray-400">
                    "Understanding the principles and design decisions behind Momenta."
                </p>
            </header>

            <section class="prose prose-gray dark:prose-invert max-w-none">
                <h2 class="font-bold uppercase">Core Principles</h2>

                <h3>1. Element-Level Reactivity</h3>
                <p>
                    "Momenta uses element-level reactivity, which means only the specific parts of your UI that depend on
                    changed data will be updated. This is more efficient than virtual DOM diffing and provides consistent performance."
                </p>

                <h3>2. Rust-First Design</h3>
                <p>
                    "Rather than porting concepts from JavaScript frameworks, Momenta embraces Rust's ownership model and
                    type system. This leads to better performance and fewer runtime errors."
                </p>

                <h3>3. Explicit Reactivity</h3>
                <p>
                    "Reactivity is explicit in Momenta. You explicitly create signals, effects, and resources. This makes
                    the reactive system predictable and debuggable."
                </p>

                <h3>4. Composability Over Configuration</h3>
                <p>
                    "Momenta provides primitive building blocks that can be composed to create complex applications.
                    There's no magic configuration or conventions - just composable primitives."
                </p>

                <h2 class="font-bold uppercase">Why Not Virtual DOM?</h2>
                <p>
                    "Virtual DOM was designed to solve a specific problem: making imperative DOM updates manageable.
                    However, with element-level reactivity, we can track exactly what changed and update the DOM directly."
                </p>

                <h2 class="font-bold uppercase">Comparison with Other Frameworks</h2>
                <p>
                    "Momenta draws inspiration from SolidJS's reactivity model but implements it in Rust with zero-cost
                    abstractions. Unlike React, there are no re-renders or reconciliation phases."
                </p>

                <Note variant="tip">
                    <p>
                        <strong>Performance:</strong> Because Momenta compiles to efficient native code and uses
                        element-level updates, your applications will be fast by default.
                    </p>
                </Note>
            </section>
        </article>
    }
}

#[component]
fn RsxPage() -> Node {
    rsx! {
        <article class="mx-auto max-w-4xl px-4 py-12 sm:px-6 lg:px-8">
            <header class="mb-12">
                <h1 class="text-4xl font-bold text-gray-900 dark:text-gray-100">rsx!</h1>
                <p class="mt-4 text-lg text-gray-600 dark:text-gray-400">
                    rsx! is a built in macro that allows you to write HTML-like syntax inside Rust code.
                    "It's a way to declaratively describe the structure of your UI."
                </p>
            </header>
            <section class="prose prose-gray dark:prose-invert max-w-none">
                <h2 id="introduction">Introduction</h2>
                <p>
                    "RSX allows you to write HTML-like syntax inside Rust code. It's a way to declaratively describe the structure of your UI."
                </p>
                <h2 id="basic-example">Basic Example</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::prelude::*;

#[component]
fn HelloWorld() -> Node {
    let name = create_signal("World");
    
    rsx! {
        <div>
            <h1>"Hello, " {name} "!"</h1>
            <p>Welcome to Momenta.</p> // see how quotes are totally optional?
            <p>"😉"</p> // N/B: currently, Momenta requires quotes for emojis
        </div>
    }
}"#}
                />
                <Note variant="info">
                    <p>
                        <strong>"Good to know:"</strong> " The rsx! macro returns a Node type that can be rendered to the DOM. Nodes are lightweight and can be composed together to build complex UIs."
                    </p>
                </Note>
                <h2 id="api-reference">API Reference</h2>
                <h3 id="creating-elements">Creating Elements</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Basic element
let div = rsx! { <div></div> };

// Element with attributes
let button = rsx! { <button type="button" class="primary"></button> };

// Self-closing element
let input = rsx! { <input type="text" /> };

// Note: HTML attributes with hyphens use underscores in RSX
// e.g., `data-id` becomes `data_id`
let custom = rsx! { <div data_id="123"></div> };

// Attributes that conflict with Rust keywords use trailing underscore
// e.g., `type` becomes `type_`
let input = rsx! { <input type="text" /> };"#}
                />
                <h3 id="attributes">Dynamic Attributes</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Dynamic class names
let is_active = create_signal(true);
let element = rsx! {
    <div class={format!("container {}", if is_active.get() { "active" } else { "" })}>
        <p>"Hello, world!"</p>
    </div>
};

// Conditional attributes
let disabled = create_signal(false);
let button = rsx! {
    <button 
        class="btn"
        disabled={disabled.get()}
    >
        "Submit"
    </button>
};

// Event handlers
let count = create_signal(0);
let button = rsx! {
    <button on:click={move |_| count += 1}>
        "Clicked " {count} " times"
    </button>
};"#}
                />
                <h3 id="children">Dynamic Children</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Text nodes
let name = "World";
let element = rsx! {
    <div>
        "Hello, " {name} "!"
    </div>
};

// Signal values
let count = create_signal(0);
let element = rsx! {
    <div>
        "Count: " {count}
    </div>
};

// Conditional rendering with when! macro
let is_logged_in = create_signal(true);
let element = rsx! {
    <div>
        {when!(is_logged_in =>
            <p>"Welcome back!"</p>
        else
            <p>"Please log in"</p>
        )}
    </div>
};

// Lists with iterators
let items = create_signal(vec!["Apple", "Banana", "Cherry"]);
let list = rsx! {
    <ul>
        {items.map(|item| rsx! {
            <li>{item}</li>
        })}
    </ul>
};"#}
                />

                <h3 id="fragments">Fragments</h3>
                <p>"When you need to return multiple elements without a wrapper, you can use fragments:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Using fragments
let elements = rsx! {
    <>
        <h1>"Title"</h1>
        <p>"Paragraph 1"</p>
        <p>"Paragraph 2"</p>
    </>
};"#}
                />

                <h2 class="font-bold uppercase">Best Practices</h2>
                <ul>
                    <li>"Keep your components small and focused on a single responsibility"</li>
                    <li>"Use signals for state that changes over time"</li>
                    <li>"Extract repeated patterns into reusable components"</li>
                    <li>"Use the when! macro for conditional rendering"</li>
                    <li>"Use iterators with .map() for rendering lists"</li>
                </ul>

                <div class="mt-12 flex items-center justify-between border-t border-gray-200 dark:border-gray-800 pt-6">
                    <a href="#" class="text-sm text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200">
                        "← Getting Started"
                    </a>
                    <a href="#" class="text-sm text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200">
                        "Signals →"
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
                    "Effects are functions that run when their dependencies change. They are the building blocks of reactivity in Momenta. Effects automatically track any signals accessed during their execution and re-run when those signals change."
                </p>
                <h2 id="basic-example">Basic Example</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::prelude::*;

#[component]
fn Counter() -> Node {
    let count = create_signal(0);
    
    // This effect will run whenever count changes
    create_effect(move || {
        log!("Count changed to: {}", count.get());
    });
    
    rsx! {
        <div>
            <p>"Current count: " {count}</p>
            <button on:click={move |_| count += 1}>"Increment"</button>
        </div>
    }
}"#}
                />
                <h2 id="api-reference">API Reference</h2>
                <h3 id="creating-effects">Creating Effects</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Basic effect creation
let name = create_signal("Alice".to_string());

// This effect will run once immediately and then
// whenever any of its dependencies change
create_effect(move || {
    log!("Hello, {}", name.get());
});

// Effects can access multiple signals
let count = create_signal(0);
let multiplier = create_signal(2);

create_effect(move || {
    let result = count.get() * multiplier.get();
    log!("Result: {}", result);
});"#}
                />
                <h2 class="font-bold uppercase">Advanced Patterns</h2>
                <h3>Effect Dependencies</h3>
                <p>"Effects can depend on multiple signals, and they will only run when any of their dependencies change:"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Effect with multiple dependencies
let first_name = create_signal("John".to_string());
let last_name = create_signal("Doe".to_string());

create_effect(move || {
    // This effect depends on both first_name and last_name
    let full_name = format!("{} {}", first_name.get(), last_name.get());
    log!("Full name: {}", full_name);
});

// Changing either signal will trigger the effect
first_name.set("Jane".to_string()); // Effect runs
last_name.set("Smith".to_string()); // Effect runs again"#}
                />
                <h3>Effect Ordering</h3>
                <p>"Effects are executed in the order they are created"</p>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"// Effect ordering
let count = create_signal(0);

create_effect(move || {
    log!("Effect 1: {}", count.get());
});

create_effect(move || {
    log!("Effect 2: {}", count.get());
});

// When count changes, the output will be:
// Effect 1: 1
// Effect 2: 1"#}
                />
                <h2 class="font-bold uppercase">Best Practices</h2>
                <ul>
                    <li>"Keep effects as lightweight as possible"</li>
                    <li>"Avoid creating effects inside loops or other complex logic"</li>
                    <li>"Use " <code>"create_effect"</code> " for simple effects"</li>
                    <li>"Don't modify signals that you're tracking in the same effect to avoid infinite loops"</li>
                    <li>"Group related effects together for better code organization"</li>
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
            <button on:click={move |_| count += 1}>
                "Increment"
            </button>
        </div>
    }
}"#}
                />

                <Note variant="info">
                    <p>
                        <strong>"Good to know:"</strong> " Unlike other frameworks, You  accessing a signal's value requires calling "
                        <code>".get()"</code> ". This explicit call enables Momenta's element-level reactivity system to track dependencies precisely."
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
    log!("Count is: {}", count);
});

// Use with closures
let doubled = move || count * 2;"#}
                />

                <h3 id="updating-values">Updating Values</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"let mut count = create_signal(0);

// Override the value
count.set(5); // Now count is 5

// Update based on previous value
count += 1; // Now count is 6
"#}
                />
                <Note variant="tip">
                    <p>
                        <strong>"Performance tip:"</strong> Avoid creating derived signals for every possible combination of signals.
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
let are_equal = a == b; // true
"#}
                />

                <h2 class="font-bold uppercase">Best Practices</h2>
                <ul>
                    <li>"Keep signals at the appropriate scope - not everything needs to be global state"</li>
                    <li>"Prefer fine-grained signals over large state objects for better performance"</li>
                    <li>"Group related signals into structs for better organization"</li>
                    <li>"Use derived values (closures that read signals) instead of creating redundant signals"</li>
                    <li>"Consider using custom signal types for domain-specific state management"</li>
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
                <h2 class="font-bold uppercase">Installation</h2>
                <p>"Add Momenta to your " <code>"Cargo.toml"</code> ":"</p>
                <CodeBlock
                    filename="Cargo.toml"
                    highlight=""
                    language="toml"
                    code={r#"[dependencies]
momenta = "0.2"

# For web projects
[dependencies.web-sys]
version = "0.3"
features = ["Document", "Element", "HtmlElement"]"#}
                />

                <h2 class="font-bold uppercase">Your First Component</h2>
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
            <h1>Hello, {name}!</h1>
            <input 
                type="text"
                value={name}
                on:input={move |e| name.set(e.value())}
                placeholder="Enter your name"
            />
        </div>
    }
}

fn main() {
    mount_to_body::<App>();
}"#}
                />

                <h2 class="font-bold uppercase">Project Structure</h2>
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
                        <strong>"Tip:"</strong> add a <code>"static"</code> folder to your project to serve static files like images, CSS, and JavaScript.
                    </p>
                </Note>
            </section>
        </article>
    }
}

#[component]
fn ComponentsPage() -> Node {
    rsx! {
        <article class="mx-auto max-w-4xl px-4 py-12 sm:px-6 lg:px-8">
            <header class="mb-12">
                <h1 class="text-4xl font-bold text-gray-900 dark:text-gray-100">Components</h1>
                <p class="mt-4 text-lg text-gray-600 dark:text-gray-400">
                    "Components are reusable pieces of UI logic marked with the #[component] attribute."
                </p>
            </header>

            <section class="prose prose-gray dark:prose-invert max-w-none">
                <h2 class="font-bold uppercase">Introduction</h2>
                <p>
                    "Components in Momenta are functions that return a Node. They can accept props and maintain
                    internal state using signals and other reactive primitives."
                </p>

                <h2 class="font-bold uppercase">Basic Component</h2>
                <CodeBlock
                    language="rust"
                    filename="src/components/button.rs"
                    highlight=""
                    code={r#"use momenta::prelude::*;

#[component]
fn Button() -> Node {
    rsx! {
        <button class="btn">
            "Click me"
        </button>
    }
}

// Usage
#[component]
fn App() -> Node {
    rsx! {
        <div>
            <Button />
        </div>
    }
}"#}
                />

                <h2 class="font-bold uppercase">Components with Props</h2>
                <CodeBlock
                    language="rust"
                    filename="src/components/button.rs"
                    highlight=""
                    code={r#"pub struct ButtonProps {
    pub text: &'static str,
    pub variant: &'static str,
    pub on:click: Box<dyn Fn()>,
}

#[component]
fn Button(props: &ButtonProps) -> Node {
    let class = format!("btn btn-{}", props.variant);
    
    rsx! {
        <button class={class} on:click={props.on:click}>
            {props.text}
        </button>
    }
}

// Usage
#[component]
fn App() -> Node {
    let count = create_signal(0);
    
    rsx! {
        <div>
            <p>Count: {count}</p>
            <Button 
                text="Increment"
                variant="primary"
                on:click={move || count += 1}
            />
        </div>
    }
}"#}
                />
                <Note variant="info">
                    <p>
                        <strong>"Good to know:"</strong> " Props must always be passed by reference or excluded totally. This ensures that the component can be re-rendered when the props change. "
                    </p>
                </Note>

                <h2 class="font-bold uppercase">Components with State</h2>
                <CodeBlock
                    language="rust"
                    filename="src/components/toggle.rs"
                    highlight=""
                    code={r#"#[component]
fn Toggle() -> Node {
    let is_on = create_signal(false);
    
    let toggle = move |_| {
        is_on.set(!is_on);
    };
    
    rsx! {
        <div class="toggle">
            <button 
                class={when!(is_on => "toggle-on" else "toggle-off")}
                on:click={toggle}
            >
                {when!(is_on => "On" else "Off")}
            </button>
        </div>
    }
}"#}
                />

                <h2 class="font-bold uppercase">Component Composition</h2>
                <CodeBlock
                    language="rust"
                    filename="src/components/card.rs"
                    highlight=""
                    code={r#"pub struct CardProps {
    pub title: &'static str,
    pub children: Vec<Node>,
}

#[component]
fn Card(props: &CardProps) -> Node {
    rsx! {
        <div class="card">
            <div class="card-header">
                <h3>{&props.title}</h3>
            </div>
            <div class="card-body">
                {&props.children}
            </div>
        </div>
    }
}

// Usage
#[component]
fn App() -> Node {
    rsx! {
        <Card title="User Profile">
            <p>"Name: John Doe"</p>
            <p>"Email: john@example.com"</p>
            <Button text="Edit Profile" variant="secondary" />
        </Card>
    }
}"#}
                />

                <h2 class="font-bold uppercase">Best Practices</h2>
                <ul>
                    <li>"Keep components focused on a single responsibility"</li>
                    <li>"Use props for data that changes between instances"</li>
                    <li>"Use signals for component-local state"</li>
                    <li>"Prefer composition over complex prop drilling"</li>
                    <li>"Name components with PascalCase"</li>
                </ul>

                <Note variant="tip">
                    <p>
                        <strong>"Performance:"</strong> " Components in Momenta have minimal overhead.
                        They're just functions that return JSX, so don't hesitate to break your UI into small, reusable pieces."
                    </p>
                </Note>
            </section>
        </article>
    }
}

#[component]
fn ShowPage() -> Node {
    rsx! {
        <article class="mx-auto max-w-4xl px-4 py-12 sm:px-6 lg:px-8">
            <header class="mb-12">
                <h1 class="text-4xl font-bold text-gray-900 dark:text-gray-100">Conditional Rendering</h1>
                <p class="mt-4 text-lg text-gray-600 dark:text-gray-400">
                    "Use when! macro for conditional rendering based on reactive values."
                </p>
            </header>

            <section class="prose prose-gray dark:prose-invert max-w-none">
                <h2 class="font-bold uppercase">Introduction</h2>
                <p>
                    "The when! macro provides a clean way to conditionally render different UI based on
                    reactive values. It's similar to ternary operators but integrates seamlessly with Momenta's reactivity."
                </p>

                <h2 class="font-bold uppercase">Basic Usage</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"use momenta::prelude::*;

#[component]
fn App() -> Node {
    let is_logged_in = create_signal(false);
    
    rsx! {
        <div>
            {when!(is_logged_in =>
                <div>
                    <h1>Welcome back!</h1>
                    <button on:click={move |_| is_logged_in.set(false)}>
                        "Logout"
                    </button>
                </div>
            else
                <div>
                    <h1>Please log in</h1>
                    <button on:click={move |_| is_logged_in.set(true)}>
                        "Login"
                    </button>
                </div>
            )}
        </div>
    }
}"#}
                />

                <h2 class="font-bold uppercase">Complex Conditions</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"let user_role = create_signal("guest");
let is_loading = create_signal(false);

rsx! {
    <div>
        {when!(is_loading =>
            <div class="spinner">Loading...</div>
        else when!(user_role == "admin" =>
            <AdminPanel />
        else when!(user_role == "user" =>
            <UserDashboard />
        else
            <GuestLanding />
        )))}
    </div>
}"#}
                />

                <h2 class="font-bold uppercase">Show Components</h2>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"pub struct ShowProps {
    pub when: bool,
    pub children: Vec<Node>,
}

#[component]
fn Show(props: &ShowProps) -> Node {
    if props.when {
        rsx! { <div>{&props.children}</div> }
    } else {
        rsx! { <div></div> }
    }
}

// Usage
let show_details = create_signal(false);

rsx! {
    <div>
        <button on:click={move |_| show_details.set(!show_details)}>
            "Toggle Details"
        </button>
        <Show when={show_details.get()}>
            <p>These are the details!</p>
            <p>Only visible when show_details is true.</p>
        </Show>
    </div>
}"#}
                />

                <h2 class="font-bold uppercase">Advanced Patterns</h2>

                <h3>Loading States</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"#[derive(Clone, Copy, PartialEq)]
enum LoadingState {
    Idle,
    Loading,
    Success,
    Error,
}

let state = create_signal(LoadingState::Idle);

rsx! {
    <div>
        {when!(state.get() {
            LoadingState::Loading => <div class="loading">
                <i class="fas fa-spinner fa-spin"></i>
                " Loading..."
            </div>,
            LoadingState::Success => <div class="success">
                <i class="fas fa-check"></i>
                " Success!"
            </div>,
            LoadingState::Error => <div class="error">
                <i class="fas fa-exclamation-triangle"></i>
                " Something went wrong"
            </div>,
            _ => <button on:click={move |_| state.set(LoadingState::Loading)}>
                "Start Operation"
            </button>
        })}
    </div>
}"#}
                />

                <h3>Permission-Based Rendering</h3>
                <CodeBlock
                    language="rust"
                    filename="src/main.rs"
                    highlight=""
                    code={r#"pub struct PermissionProps {
    pub required_permission: &'static str,
    pub children: Vec<Node>,
}

#[component]
fn RequirePermission(props: &PermissionProps) -> Node {
    let user_permissions = use_context::<Vec<String>>();
    
    let has_permission = user_permissions
        .iter()
        .any(|p| p == props.required_permission);
    
    rsx! {
        {when!(has_permission =>
            <div>{&props.children}</div>
        else
            <div class="permission-denied">
                "You don't have permission to view this content."
            </div>
        )}
    }
}

// Usage
rsx! {
    <RequirePermission required_permission="admin">
        <AdminSettings />
    </RequirePermission>
}"#}
                />

                <h2 class="font-bold uppercase">Best Practices</h2>
                <ul>
                    <li>"Use when! for simple boolean conditions"</li>
                    <li>"Consider creating Show/Hide components for reusable patterns"</li>
                    <li>"Avoid deeply nested conditional rendering"</li>
                    <li>"Use match expressions for complex state-based rendering"</li>
                    <li>"Keep condition logic readable and maintainable"</li>
                </ul>
            </section>
        </article>
    }
}

#[component]
fn CounterExample() -> Node {
    let mut count = create_signal(0);

    rsx! {
        <div class="min-h-full bg-gradient-to-br from-purple-400 to-blue-600 flex items-center justify-center p-4">
            <div class="bg-white/20 backdrop-blur-lg rounded-3xl p-8 shadow-2xl border border-white/30">
                <h1 class="text-3xl font-bold text-white mb-6 text-center">
                    "Momenta Counter"
                </h1>

                <div class="text-6xl font-bold text-center mb-8 transition-all duration-300 text-white">
                    {count}
                </div>

                <div class="flex gap-4 justify-center">
                    <button
                        class="px-6 py-3 bg-red-500 hover:bg-red-600 text-white font-semibold rounded-xl transition-all duration-200 transform hover:scale-105 shadow-lg"
                        on:click={move |_| count -= 1}
                    >
                        "− Decrease"
                    </button>

                    <button
                        class="px-6 py-3 bg-green-500 hover:bg-green-600 text-white font-semibold rounded-xl transition-all duration-200 transform hover:scale-105 shadow-lg"
                        on:click={move |_| count += 1}
                    >
                        "+ Increase"
                    </button>
                </div>

                <button
                    class="w-full mt-4 px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg transition-colors"
                    on:click={move |_| count.set(0)}
                >
                    "Reset Count: " {count}
                </button>
            </div>
        </div>
    }
}

fn main() {
    render_root::<App>("#app");
}
