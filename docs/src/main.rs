#![no_std]
#![allow(unused_braces)]

extern crate alloc;
use alloc::vec;

use simple_rsx::Node;
use simple_rsx::component;
use simple_rsx::dom::render_root;
use simple_rsx::either;
use simple_rsx::rsx;
use simple_rsx::signals::SignalValue;
use simple_rsx::signals::create_signal;

mod components;
mod effects;
mod resources;
mod signals;
use components::*;
use effects::EffectsPage;
use resources::ResourcesPage;
use signals::SignalsPage;

#[derive(Clone, Copy, PartialEq)]
enum Page {
    Landing,
    Installation,
    GetStarted,
    Counter,
    Concepts,
    Signals,
    Resources,
    Effects,
}

impl SignalValue for Page {
    fn as_any(&self) -> Option<&dyn core::any::Any> {
        Some(self)
    }
}

#[component]
fn CounterAppPage() -> Node {
    let counter_example = r#"use momenta::prelude::*;
#[component]
fn Counter() -> Node {
    let mut count = create_signal(0);
    
    rsx! {
        <div class="text-center p-6">
            <h2 class="text-3xl font-bold mb-4">"Count: {count()}"</h2>
            <button 
                on_click={move |_| count += 1}
                class="px-4 py-2 bg-primary-600  text-white rounded-md">
                "Increment"
            </button>
        </div>
    }
}"#;

    rsx! {
        <section class="py-20 bg-gradient-to-br from-primary-50 via-background to-secondary-50 dark:from-primary-950 dark:via-background dark:to-secondary-950 animate-in">
            <div class="container mx-auto">
                <div class="max-w-4xl mx-auto rounded-2xl shadow-xl glass-effect p-10 md:p-16">
                    <PageHeader title="Examples" subtitle="Learn by example with practical code snippets" />
                    <div class="prose prose-slate max-w-none">
                        <ContentSection title="Basic Counter" icon="fas fa-play">
                            <div>
                                <p class="text-muted-foreground mb-4 text-lg">
                                    A simple counter demonstrating signals and event handling:
                                </p>
                                <CodeBlock title="counter.rs" code={counter_example} language="rust" />
                                <InfoBox
                                    icon="fas fa-lightbulb"
                                    title="Key Features: "
                                    content="Signal creation, event handlers, reactive updates"
                                    variant="info"
                                />
                            </div>
                        </ContentSection>
                        <KeyConceptsGrid />
                        <div class="mt-12 flex justify-center">
                            <a
                                href="#concepts"
                                class="inline-flex items-center justify-center rounded-lg text-base font-semibold transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-background bg-primary-600 text-primary-foreground hover:bg-primary-700/90 shadow-lg h-12 py-3 px-6 gap-2 animate-in"
                            >
                                <i class="fas fa-arrow-right"></i>
                                <span>Explore Core Concepts</span>
                            </a>
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
        <section class="py-20 bg-gradient-to-br from-secondary-50 via-background to-primary-50 dark:from-secondary-950 dark:via-background dark:to-primary-950 animate-in">
            <div class="container">
                <div class="max-w-4xl mx-auto rounded-2xl shadow-xl glass-effect p-10 md:p-16">
                    <PageHeader title="Core Concepts" subtitle="Master the fundamental building blocks of Momenta" />
                    <div class="prose max-w-none">
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-8 mb-12">
                            <ConceptCard
                                icon="fas fa-broadcast-tower"
                                title="Signals"
                                description="Reactive state management through signals. Components automatically re-render when signals they depend on change."
                                benefits={vec![
                                    ("Fine-grained reactivity:", "Only affected components re-render"),
                                    ("Automatic dependency tracking:", "No manual subscriptions needed"),
                                    ("Minimal re-renders:", "Optimized performance by default"),
                                    ("Thread-safe by default:", "Built for concurrent environments")
                                ]}
                                color="primary"
                            />
                            <ConceptCard
                                icon="fas fa-magic"
                                title="Effects"
                                description="Side effects and subscriptions that run in response to signal changes. Perfect for API calls, timers, and cleanup logic."
                                benefits={vec![
                                    ("Data fetching:", "Automatic API calls on state changes"),
                                    ("DOM manipulation:", "Direct DOM updates when needed"),
                                    ("Event subscriptions:", "WebSocket and external event handling"),
                                    ("Resource cleanup:", "Automatic cleanup on component unmount")
                                ]}
                                color="blue"
                            />
                            <ConceptCard
                                icon="fas fa-layer-group"
                                title="Rendering Scopes"
                                description="Isolated rendering contexts that manage component lifecycles and cleanup automatically. Each scope owns its signals and effects."
                                benefits={vec![
                                    ("Automatic cleanup:", "Memory leaks are prevented"),
                                    ("Memory safety:", "Rust's ownership ensures correctness"),
                                    ("Hierarchical structure:", "Parent-child scope relationships"),
                                    ("Performance isolation:", "Scoped optimization boundaries")
                                ]}
                                color="green"
                            />
                            <ConceptCard
                                icon="fas fa-puzzle-piece"
                                title="Components and Props"
                                description="Reusable UI components with type-safe props. Components are just Rust functions that return renderable content."
                                benefits={vec![
                                    ("Type safety:", "Compile-time prop validation"),
                                    ("Compile-time validation:", "Catch errors before runtime"),
                                    ("Zero-cost abstractions:", "No runtime overhead"),
                                    ("Easy composition:", "Mix and match components freely")
                                ]}
                                color="secondary"
                            />
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

// Main App Component
#[component]
fn App() -> Node {
    let current_page = create_signal(Page::Landing);
    let theme = create_signal("dark");

    rsx! {
        <div class="min-h-screen flex flex-col bg-background text-foreground">
            <Header current_page={current_page} theme={theme} />

            <div class="flex-grow flex">

                <main class="flex-grow">
                    {
                        match current_page.get() {
                            Page::Landing => rsx! { <LandingPage /> },
                            Page::Installation => rsx! { <InstallationPage /> },
                            Page::GetStarted => rsx! { <GetStartedPage /> },
                            Page::Counter => rsx! { <CounterAppPage /> },
                            Page::Concepts => rsx! { <ConceptsPage /> },
                            Page::Signals => rsx! { <SignalsPage /> },
                            Page::Resources => rsx! { <ResourcesPage /> },
                            Page::Effects => rsx! { <EffectsPage /> },
                        }
                    }
                </main>
                {either!(current_page != Page::Landing => <Sidebar current_page={current_page} />)}
            </div>
            <Footer />
        </div>
    }
}

#[component]
fn LandingPage() -> Node {
    rsx! {
        <div>
            <HeroSection />
            <FeaturesSection />
            <CallToActionSection />
        </div>
    }
}

#[component]
fn HeroSection() -> Node {
    let counter_example = r#"use momenta::prelude::*;

#[component]
fn Counter() -> Node {
    let mut count = create_signal(0);
    
    rsx! {
        <div class="text-center p-6">
            <h2 class="text-3xl font-bold mb-4">"Count: {count}"</h2>
            <button 
                on_click={move |_| count += 1}
                class="px-4 py-2 bg-primary-600  text-white rounded-md">
                "Increment"
            </button>
        </div>
    }
}"#;

    rsx! {
        <section class="bg-gradient-to-br from-primary-600  to-secondary">
            <div class="container py-20 md:py-28">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-12 items-center">
                    <div>
                        <h1 class="text-4xl md:text-5xl font-bold text-white mb-4">Momenta</h1>
                        <h2 class="text-xl md:text-2xl font-medium text-white/90 mb-6">
                            A reactive UI metaframework for Rust
                        </h2>
                        <p class="text-white/80 mb-8 text-lg">
                            Build high-performance web applications with fine-grained reactivity and a familiar JSX-like syntax.
                        </p>
                        <div class="flex flex-wrap gap-4">
                            <a
                                href="#get-started"
                                class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-background bg-white text-primary-600  hover:bg-white/90 h-10 py-2 px-4"
                            >
                                <i class="fas fa-arrow-right mr-2"></i>
                                <span>Get Started</span>
                            </a>
                            <a
                                href="https://github.com/elcharitas/simple-rsx"
                                class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-background border border-white text-white hover:bg-white/10 h-10 py-2 px-4"
                                target="_blank"
                            >
                                <i class="fab fa-github mr-2"></i>
                                <span>GitHub</span>
                            </a>
                        </div>
                    </div>
                    <div>
                        <CodeBlock title="Counter.rs" code={counter_example} language="rust" />
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn FeaturesSection() -> Node {
    rsx! {
        <section class="py-20 bg-background">
            <div class="container">
                <div class="text-center mb-16">
                    <h2 class="text-3xl font-bold mb-4">Why Momenta?</h2>
                    <p class="text-lg text-muted-foreground max-w-3xl mx-auto">
                        A lightweight, reactive framework for building modern web applications in Rust
                    </p>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                    <FeatureCard
                        icon="fas fa-bolt"
                        title="Fine-Grained Reactivity"
                        description="Precise updates with minimal overhead using a signal-based reactive system."
                    />
                    <FeatureCard
                        icon="fas fa-code"
                        title="RSX Syntax"
                        description="Familiar JSX-like syntax that makes UI development in Rust intuitive and enjoyable."
                    />
                    <FeatureCard
                        icon="fas fa-shield-alt"
                        title="Type Safe"
                        description="Leverage Rust's strong type system to catch errors at compile time rather than runtime."
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn CallToActionSection() -> Node {
    rsx! {
        <section class="py-20 bg-gradient-to-br from-primary-600/90 to-secondary/90">
            <div class="container text-center">
                <h2 class="text-3xl font-bold text-white mb-4">Ready to get started?</h2>
                <p class="text-lg text-white/80 mb-8 max-w-3xl mx-auto">
                    Join the growing community of developers building with Momenta.
                </p>
                <div class="flex flex-wrap justify-center gap-4">
                    <a
                        href="#installation"
                        class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-background bg-white text-primary-600  hover:bg-white/90 h-10 py-2 px-4"
                    >
                        <i class="fas fa-download mr-2"></i>
                        <span>Installation Guide</span>
                    </a>
                    <a
                        href="#"
                        class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-background border border-white text-white hover:bg-white/10 h-10 py-2 px-4"
                    >
                        <i class="fas fa-book mr-2"></i>
                        <span>API Documentation</span>
                    </a>
                </div>
            </div>
        </section>
    }
}

#[component]
fn InstallationPage() -> Node {
    let cargo_toml_code = r#"[dependencies]
momenta = "0.1.0"#;

    let terminal_code = "$ cargo add momenta";

    let main_rs_code = r#"use momenta::prelude::*;

#[component]
fn App() -> Node {
    rsx! {
        <div>"Hello, Momenta!"</div>
    }
}

fn main() {
    render_root::<App>("app");
}"#;

    rsx! {
        <div class="container py-12">
            <PageHeader title="Installation" subtitle="Get Momenta up and running in your project" />

            <div class="prose prose-slate max-w-none">
                <ContentSection title="Prerequisites" icon="fas fa-info-circle">
                    <div>
                        <p class="text-muted-foreground mb-4">
                            Make sure you have Rust installed on your system. If not, install it from
                            <a href="https://rustup.rs/" target="_blank" class="text-primary-600  hover:underline">rustup.rs</a>.
                        </p>
                        <InfoBox
                            icon="fas fa-info-circle"
                            title="Minimum Rust version: "
                            content="1.70+"
                            variant="info"
                        />
                    </div>
                </ContentSection>

                <ContentSection title="Add to Your Project" icon="fas fa-plus">
                    <div>
                        <p class="text-muted-foreground mb-4">
                            Add Momenta to your <code class="px-1.5 py-0.5 bg-muted rounded">Cargo.toml</code>:
                        </p>
                        <CodeBlock title="Cargo.toml" code={cargo_toml_code} language="toml" />

                        <p class="text-muted-foreground mb-4">"Or use cargo add:"</p>
                        <CodeBlock title="Terminal" code={terminal_code} language="bash" />
                    </div>
                 </ContentSection>

                <ContentSection title="Feature Flags" icon="fas fa-flag">
                        <div>
                            <p class="text-muted-foreground mb-4">Momenta supports several optional features:</p>
                            <FeatureTable />
                        </div>
                </ContentSection>

                <ContentSection title="Verify Installation" icon="fas fa-check-circle">
                    <div>
                        <p class="text-muted-foreground mb-4">Create a simple test to verify everything works:</p>
                        <CodeBlock title="main.rs" code={main_rs_code} language="rust" />
                        <InfoBox
                            icon="fas fa-check-circle"
                            title="Success! "
                            content="You're ready to start building with Momenta."
                            variant="success"
                        />
                    </div>
                </ContentSection>

                <div class="mt-6">
                    <a
                        href="#get-started"
                        class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-background bg-primary-600  text-primary-foreground hover:bg-primary-600/90 h-10 py-2 px-4"
                    >
                        <i class="fas fa-arrow-right mr-2"></i>
                        <span>Getting Started Guide</span>
                    </a>
                </div>
            </div>
        </div>
    }
}

#[component]
fn GetStartedPage() -> Node {
    let basic_app_code = r#"use momenta::prelude::*;

#[component]
fn App() -> Node {
    let mut count = signal(0);
    
    let increment = move |_| {
        count += 1;
    };
    
    rsx! {
        <div>
            <h1>Count: {count.get()}</h1>
            <button onclick={increment}>Increment</button>
        </div>
    }
}

fn main() {
    momenta::render(App);
}"#;

    let styling_code = r#"rsx! {
    <div class="p-4 bg-blue-100 rounded">
        <h1 class="text-2xl font-bold text-blue-800">Count: {count.get()}</h1>
        <button 
            class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600" 
            onclick={increment}
        >
            Increment
        </button>
    </div>
}"#;

    rsx! {
        <div class="container py-12">
            <PageHeader
                title="Get Started"
                subtitle="Learn Momenta fundamentals step by step"
            />

            <div class="prose prose-slate max-w-none">
                <ContentSection title="Creating Your First App" icon="fas fa-puzzle-piece">
                    <div>
                        <p class="text-muted-foreground mb-4">
                            Let&apos;s create a simple counter application to demonstrate the basics of Momenta.
                        </p>
                        <CodeBlock
                            title="src/main.rs"
                            code={basic_app_code}
                            language="rust"
                        />
                        <p class="mt-4 mb-4 text-muted-foreground">
                            This example demonstrates the core concepts of Momenta:
                        </p>
                        <ul class="list-disc pl-6 mb-6 text-muted-foreground space-y-2">
                            <li><strong>Signals</strong> - Reactive state variables that trigger updates when changed</li>
                            <li><strong>Components</strong> - Reusable UI elements defined with the <code class="bg-muted px-1 py-0.5 rounded">#[component]</code> attribute</li>
                            <li><strong>RSX</strong> - JSX-like syntax for defining UI structure</li>
                            <li><strong>Event Handling</strong> - Responding to user interactions</li>
                        </ul>
                    </div>
                </ContentSection>

                <ContentSection title="Adding Styles" icon="fas fa-palette">
                    <div>
                        <p class="text-muted-foreground mb-4">
                            Momenta supports inline CSS classes for styling your components:
                        </p>
                        <CodeBlock
                            title="Styled Component"
                            code={styling_code}
                            language="rust"
                        />
                        <p class="mt-4 text-muted-foreground">
                            You can use any CSS framework like Tailwind CSS or write your own styles.
                        </p>
                    </div>
                </ContentSection>

                <EventHandlingSection />

                <div class="mt-10 flex justify-center">
                    <a
                        href="#counter"
                        class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-background bg-primary-600  text-primary-foreground hover:bg-primary-600/90 h-10 py-2 px-4"
                    >
                        <i class="fas fa-arrow-right mr-2"></i>
                        <span>View Counter Example</span>
                    </a>
                </div>
            </div>
        </div>
    }
}

fn main() {
    render_root::<App>("app");
}
