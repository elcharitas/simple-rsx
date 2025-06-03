use alloc::{format, vec, vec::Vec};
use simple_rsx::{Node, component, either, rsx, signals::Signal};

use crate::Page;

pub struct CodeBlockProps {
    pub title: &'static str,
    pub code: &'static str,
    pub language: Option<&'static str>,
}

pub struct InfoBoxProps {
    pub icon: &'static str,
    pub title: &'static str,
    pub content: &'static str,
    pub variant: &'static str,
}

pub struct FeatureCardProps {
    pub icon: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}

pub struct PageHeaderProps {
    pub title: &'static str,
    pub subtitle: &'static str,
}

pub struct ContentSectionProps {
    pub title: &'static str,
    pub icon: Option<&'static str>,
    pub children: Vec<Node>,
}

pub struct ButtonProps {
    pub href: &'static str,
    pub text: &'static str,
    pub variant: &'static str,
    pub icon: Option<&'static str>,
    pub children: Vec<Node>,
}

pub struct TableRowProps {
    pub feature: &'static str,
    pub description: &'static str,
    pub default: &'static str,
    pub children: Vec<Node>,
}

pub struct HeaderProps {
    pub current_page: Signal<Page>,
    pub theme: Signal<&'static str>,
}

pub struct EventTypeCardProps {
    pub title: &'static str,
    pub icon: &'static str,
    pub events: Vec<&'static str>,
    pub color: &'static str,
}

pub struct ConceptItemProps {
    pub icon: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub color: &'static str,
}

pub struct ConceptCardProps {
    pub icon: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub benefits: Vec<(&'static str, &'static str)>,
    pub color: &'static str,
}

// Reusable Components

#[component]
pub fn EventHandlingSection() -> Node {
    rsx! {
        <ContentSection title="Event Handling" icon="fas fa-mouse-pointer">
            <div>
                <p class="text-muted-foreground mb-4">
                    Handle user interactions with event handlers. Momenta supports all standard DOM events.
                </p>

                <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
                    <EventTypeCard
                        title="Mouse Events"
                        icon="fas fa-mouse"
                        events={vec!["on_click", "on_mouseenter", "on_mouseleave"]}
                        color="secondary"
                    />
                    <EventTypeCard
                        title="Keyboard Events"
                        icon="fas fa-keyboard"
                        events={vec!["on_keydown", "on_keyup", "on_keypress"]}
                        color="blue"
                    />
                    <EventTypeCard
                        title="Form Events"
                        icon="fas fa-edit"
                        events={vec!["on_input", "on_change", "on_submit"]}
                        color="green"
                    />
                </div>

                <InfoBox
                    icon="fas fa-info-circle"
                    title="Pro Tip: "
                    content="Use closures to capture variables from the surrounding scope in your event handlers."
                    variant="success"
                />
            </div>
        </ContentSection>
    }
}

#[component]
pub fn KeyConceptsGrid() -> Node {
    rsx! {
        <ContentSection title="Key Concepts Demonstrated" icon="fas fa-graduation-cap">
            <div>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                    <ConceptItem
                        icon="fas fa-broadcast-tower"
                        title="Signal creation and updates"
                        description="Reactive state management with automatic re-rendering"
                        color="primary"
                    />
                    <ConceptItem
                        icon="fas fa-mouse-pointer"
                        title="Event handler closures"
                        description="Capturing variables in event callbacks"
                        color="primary"
                    />
                    <ConceptItem
                        icon="fas fa-eye"
                        title="Conditional rendering"
                        description="Dynamic UI based on state conditions"
                        color="secondary"
                    />
                    <ConceptItem
                        icon="fas fa-palette"
                        title="Dynamic class names"
                        description="Styling based on component state"
                        color="red"
                    />
                </div>
                <ConceptItem
                    icon="fas fa-history"
                    title="State history management"
                    description="Tracking and displaying state changes over time"
                    color="primary"
                />
            </div>
        </ContentSection>
    }
}

#[component]
pub fn CodeBlock(props: &CodeBlockProps) -> Node {
    let _lang_display = props.language.unwrap_or("");

    rsx! {
        <div class="mb-6 rounded-md overflow-hidden border border-border">
            <div class="bg-muted px-4 py-2 border-b border-border flex justify-between items-center">
                <span class="text-sm font-medium">{props.title}</span>
                <button class="text-muted-foreground hover:text-foreground transition-colors">
                    <i class="fas fa-copy"></i>
                </button>
            </div>
            <pre class="bg-code-background p-4 overflow-x-auto">
                <code class="text-sm text-code-foreground font-mono">{props.code}</code>
            </pre>
        </div>
    }
}

#[component]
pub fn InfoBox(props: &InfoBoxProps) -> Node {
    let (bg_color, border_color, text_color, icon_color) = match props.variant {
        "info" => (
            "bg-primary-600/10",
            "border-primary",
            "text-slate-700",
            "text-primary",
        ),
        "success" => (
            "bg-green-50",
            "border-green-500",
            "text-slate-700",
            "text-green-500",
        ),
        "warning" => (
            "bg-secondary-50",
            "border-secondary-500",
            "text-slate-700",
            "text-secondary-500",
        ),
        _ => (
            "bg-primary-600/10",
            "border-primary",
            "text-slate-700",
            "text-primary",
        ),
    };

    rsx! {
        <div class={format!("{} border-l-4 {} p-4 rounded-r-md", bg_color, border_color)}>
            <p class={text_color}>
                <span class={format!("{} mr-2", icon_color)}>
                    <i class={props.icon}></i>
                </span>
                <strong>{props.title}</strong>
                <span>{props.content}</span>
            </p>
        </div>
    }
}

#[component]
pub fn FeatureCard(props: &FeatureCardProps) -> Node {
    rsx! {
        <div class="p-6 bg-card rounded-lg border border-border hover:shadow-md transition-shadow">
            <div class="w-12 h-12 bg-primary-600/10 text-primary-600  rounded-full flex items-center justify-center mb-4">
                <i class={format!("{} text-xl", props.icon)}></i>
            </div>
            <h3 class="text-xl font-semibold mb-2">{props.title}</h3>
            <p class="text-muted-foreground">{props.description}</p>
        </div>
    }
}

#[component]
pub fn PageHeader(props: &PageHeaderProps) -> Node {
    rsx! {
        <div class="mb-16 text-center relative">
            <div class="absolute inset-0 -z-10 overflow-hidden">
                <div class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[400px] bg-gradient-to-br from-primary-600/20 via-secondary-600/20 to-primary-600/20 blur-3xl rounded-full animate-pulse"></div>
            </div>
            <h1 class="text-6xl font-extrabold mb-6 bg-gradient-to-r from-primary-600 via-secondary-500 to-primary-600 bg-clip-text text-transparent drop-shadow-xl animate-in slide-in">{props.title}</h1>
            <p class="text-2xl text-muted-foreground/90 font-medium max-w-2xl mx-auto leading-relaxed animate-in slide-in">{props.subtitle}</p>
        </div>
    }
}

#[component]
pub fn ContentSection(
    ContentSectionProps {
        children,
        icon,
        title,
    }: &ContentSectionProps,
) -> Node {
    rsx! {
        <div class="mb-12 border border-border/40 rounded-2xl overflow-hidden shadow-xl bg-gradient-to-b from-card/95 to-background/95 backdrop-blur-sm transition-all duration-300 hover:shadow-2xl hover:border-border/60">
            <div class="bg-gradient-to-r from-muted/40 via-muted/60 to-muted/40 px-8 py-6 border-b border-border/40">
                <h2 class="text-xl font-bold flex items-center">
                    {if let Some(icon_class) = *icon {
                        rsx! {
                            <span class="bg-gradient-to-br from-primary-600 to-secondary-600 text-white p-2.5 rounded-lg mr-4 shadow-md">
                                <i class={format!("{} text-lg", icon_class)}></i>
                            </span>
                        }
                    } else {
                        rsx! { <span></span> }
                    }}
                    <span class="bg-gradient-to-r from-primary-600 to-secondary-600 bg-clip-text text-transparent">{title}</span>
                </h2>
            </div>
            <div class="p-8 bg-card/30">
                {children}
            </div>
        </div>
    }
}

#[component]
pub fn Button(props: &ButtonProps) -> Node {
    let class = match props.variant {
        "primary" => {
            "inline-flex items-center px-5 py-3 border border-transparent text-base font-medium rounded-md text-white bg-primary-600  hover:bg-primary-dark transition-colors"
        }
        "secondary" => {
            "inline-flex items-center px-5 py-3 border border-white text-base font-medium rounded-md text-white hover:bg-white/10 transition-colors"
        }
        "outline" => {
            "inline-flex items-center px-3 py-1.5 border border-slate-300 text-sm font-medium rounded-md text-slate-700 bg-white hover:bg-slate-50"
        }
        _ => {
            "inline-flex items-center px-5 py-3 border border-transparent text-base font-medium rounded-md text-white bg-primary-600  hover:bg-primary-dark transition-colors"
        }
    };

    rsx! {
        <a class={class} href={props.href}>
            {if let Some(icon_class) = props.icon {
                rsx! {
                    <span class="mr-2">
                        <i class={icon_class}></i>
                    </span>
                }
            } else {
                rsx! { <span></span> }
            }}
            <span>{props.text}</span>
            {&props.children}
        </a>
    }
}

#[component]
pub fn TableRow(props: &TableRowProps) -> Node {
    rsx! {
        <tr>
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">{props.feature}</td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-muted-foreground">{props.description}</td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-muted-foreground font-mono text-xs bg-muted/50 rounded mx-2">{props.default}</td>
            {&props.children}
        </tr>
    }
}

#[component]
pub fn EventTypeCard(props: &EventTypeCardProps) -> Node {
    let (bg_class, border_class, badge_class) = match props.color {
        "secondary" => (
            "bg-secondary-500/10",
            "border-secondary-500/20",
            "bg-secondary-500",
        ),
        "blue" => ("bg-primary-600/10", "border-primary-600/20", "bg-primary"),
        "green" => ("bg-green-500/10", "border-green-500/20", "bg-green-500"),
        _ => ("bg-muted", "border-border", "bg-muted-foreground"),
    };

    rsx! {
        <div class={format!("{} border {} rounded-lg p-4", bg_class, border_class)}>
            <h4 class="font-semibold mb-2 flex items-center">
                <span class="mr-2">
                    <i class={props.icon}></i>
                </span>
                <span>{props.title}</span>
            </h4>
            <div class="flex flex-wrap gap-2">
                {props.events.iter().map(|event| {
                    rsx! {
                        <span class={format!("inline-block px-2 py-1 text-xs font-medium {} text-white rounded", badge_class)}>
                            {event}
                        </span>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[component]
pub fn ConceptItem(props: &ConceptItemProps) -> Node {
    let icon_color = match props.color {
        "primary" => "text-primary",
        "secondary" => "text-secondary-500",
        "red" => "text-red-500",
        _ => "text-primary",
    };

    rsx! {
        <div class="flex items-start space-x-3">
            // #region
            <span class={format!("{} mt-0.5", icon_color)}>
                <i class={props.icon}></i>
            </span>
            // #endregion
            <div>
                <p class="font-semibold">{props.title}</p>
                <p class="text-sm text-muted-foreground">{props.description}</p>
            </div>
        </div>
    }
}

#[component]
pub fn ConceptCard(props: &ConceptCardProps) -> Node {
    let icon_color = match props.color {
        "primary" => "text-primary",
        "blue" => "text-primary",
        "green" => "text-green-500",
        "secondary" => "text-secondary-500",
        _ => "text-primary",
    };

    let benefit_icon = match props.color {
        "blue" => "fas fa-wrench",
        "green" => "fas fa-shield-alt",
        "secondary" => "fas fa-check-double",
        _ => "fas fa-star",
    };

    let benefit_icon_color = match props.color {
        "blue" => "text-primary",
        "green" => "text-green-500",
        "secondary" => "text-secondary-500",
        _ => "text-primary",
    };

    let check_color = match props.color {
        "blue" => "text-primary",
        "green" => "text-green-500",
        "secondary" => "text-secondary-500",
        _ => "text-green-500",
    };

    rsx! {
        <div class="border border-border rounded-lg bg-card p-6 h-full">
            <div class="mb-5">
                <div class="flex items-center mb-3">
                    <span class={format!("{} mr-3", icon_color)}>
                        <i class={format!("{} text-xl", props.icon)}></i>
                    </span>
                    <h3 class="text-xl font-semibold">{props.title}</h3>
                </div>
                <p class="text-muted-foreground leading-relaxed">{props.description}</p>
            </div>

            <div>
                <h4 class="flex items-center font-semibold mb-3">
                    <span class={format!("{} mr-2", benefit_icon_color)}>
                        <i class={benefit_icon}></i>
                    </span>
                    {match props.color {
                        "blue" => "Use Cases:",
                        "green" => "Features:",
                        "secondary" => "Advantages:",
                        _ => "Benefits:"
                    }}
                </h4>
                <ul class="text-muted-foreground text-sm space-y-2">
                    {props.benefits.iter().map(|(label, desc)| {
                        rsx! {
                            <li class="flex items-start">
                                <span class={format!("{} mr-2 mt-0.5", check_color)}>
                                    <i class={match props.color {
                                        "blue" => "fas fa-arrow-right",
                                        _ => "fas fa-check"
                                    }}></i>
                                </span>
                                <span>
                                    <strong class="font-medium">{label}</strong>
                                    <span>{" "}{desc}</span>
                                </span>
                            </li>
                        }
                    })}
                </ul>
            </div>
        </div>
    }
}

pub struct SidebarProps {
    pub current_page: Signal<Page>,
    pub children: Vec<Node>,
}

#[component]
pub fn Sidebar(props: &SidebarProps) -> Node {
    let current_page = props.current_page;
    let nav = move |page: Page| {
        move |_| {
            current_page.set(page);
        }
    };

    let sidebar_link_class = |page: Page| {
        if current_page.get() == page {
            "flex items-center px-4 py-2.5 text-sm font-medium rounded-lg bg-gradient-to-r from-primary-600/10 to-secondary-600/10 text-primary border border-primary/10 shadow-sm"
        } else {
            "flex items-center px-4 py-2.5 text-sm font-medium rounded-lg hover:bg-muted/80 text-foreground hover:text-primary-600 transition-all duration-200 ease-in-out hover:shadow-sm hover:translate-x-1"
        }
    };

    rsx! {
        <div class="hidden md:block w-72 shrink-0 border-r border-border/40 bg-card/50 backdrop-blur-xl shadow-xl">
            <div class="h-full py-8 px-4">
                <div class="space-y-1">
                    <h3 class="px-3 text-sm font-medium text-muted-foreground mb-2">Getting Started</h3>
                    <a class={sidebar_link_class(Page::Installation)} href="#installation" on_click={nav(Page::Installation)}>
                        <i class="fas fa-download mr-2 text-xs opacity-70"></i>
                        <span>Installation</span>
                    </a>
                    <a class={sidebar_link_class(Page::GetStarted)} href="#get-started" on_click={nav(Page::GetStarted)}>
                        <i class="fas fa-play mr-2 text-xs opacity-70"></i>
                        <span>Get Started</span>
                    </a>
                </div>

                <div class="mt-6 space-y-1">
                    <h3 class="px-3 text-sm font-medium text-muted-foreground mb-2">Examples</h3>
                    <a class={sidebar_link_class(Page::Counter)} href="#counter" on_click={nav(Page::Counter)}>
                        <i class="fas fa-calculator mr-2 text-xs opacity-70"></i>
                        <span>Counter App</span>
                    </a>
                </div>

                <div class="mt-6 space-y-1">
                    <h3 class="px-3 text-sm font-medium text-muted-foreground mb-2">Core</h3>
                    <a class={sidebar_link_class(Page::Concepts)} href="#concepts" on_click={nav(Page::Concepts)}>
                        <i class="fas fa-graduation-cap mr-2 text-xs opacity-70"></i>
                        <span>Core Concepts</span>
                    </a>
                    <a class={sidebar_link_class(Page::Signals)} href="#signals" on_click={nav(Page::Signals)}>
                        <i class="fas fa-broadcast-tower mr-2 text-xs opacity-70"></i>
                        <span>Signals</span>
                    </a>
                    <a class={sidebar_link_class(Page::Resources)} href="#resources" on_click={nav(Page::Resources)}>
                        <i class="fas fa-cloud-download-alt mr-2 text-xs opacity-70"></i>
                        <span>Resources</span>
                    </a>
                    <a class={sidebar_link_class(Page::Effects)} href="#effects" on_click={nav(Page::Effects)}>
                        <i class="fas fa-magic mr-2 text-xs opacity-70"></i>
                        <span>Effects</span>
                    </a>
                </div>

                <div class="mt-6 pt-6 border-t border-border">
                    <a
                        href="https://github.com/elcharitas/simple-rsx"
                        class="flex items-center px-3 py-2 text-sm font-medium hover:bg-muted text-foreground hover:text-primary-600  transition-colors"
                        target="_blank"
                    >
                        <i class="fab fa-github mr-2 text-xs opacity-70"></i>
                        <span>GitHub</span>
                    </a>
                </div>
                {&props.children}
            </div>
        </div>
    }
}

fn navbar_class(is_active: bool) -> &'static str {
    if is_active {
        "text-primary font-medium relative after:absolute after:bottom-0 after:left-0 after:w-full after:h-0.5 after:bg-gradient-to-r after:from-primary-600 after:to-secondary-600 after:rounded-full py-1"
    } else {
        "text-muted-foreground hover:text-primary relative after:absolute after:bottom-0 after:left-0 after:w-0 after:h-0.5 after:bg-gradient-to-r after:from-primary-600 after:to-secondary-600 after:rounded-full after:transition-all after:duration-300 hover:after:w-full py-1"
    }
}

#[component]
pub fn Header(
    HeaderProps {
        current_page,
        theme,
    }: &HeaderProps,
) -> Node {
    let current_page = current_page.clone();
    let theme = theme.clone();

    let nav = move |page: Page| {
        move |_| {
            current_page.set(page);
        }
    };

    let toggle_theme = move |_| {
        let current = theme.get();
        theme.set(if current == "dark" { "light" } else { "dark" });
    };

    rsx! {
        <header class="sticky top-0 z-40 w-full backdrop-blur-xl bg-background/80 border-b border-border/40 shadow-lg">
            <div class="container flex h-16 items-center">
                <a class="flex items-center mr-8 group transition-transform hover:scale-105" href="#" on_click={nav(Page::Landing)}>
                    <div class="text-primary-600 mr-3 transition-transform group-hover:rotate-12">
                        <i class="fas fa-bolt text-2xl"></i>
                    </div>
                    <div>
                        <h1 class="text-xl font-bold bg-clip-text text-white">Momenta</h1>
                    </div>
                </a>

                <nav class="hidden md:flex items-center space-x-6 lg:space-x-8">
                    <a class={format!("{} transition-all duration-200 ease-in-out", navbar_class(current_page == Page::Installation))}
                       href="#installation" on_click={nav(Page::Installation)}>
                        <i class="fas fa-download mr-2 opacity-70"></i>Installation
                    </a>
                    <a class={format!("{} transition-all duration-200 ease-in-out", navbar_class(current_page == Page::GetStarted))}
                       href="#get-started" on_click={nav(Page::GetStarted)}>
                        <i class="fas fa-play mr-2 opacity-70"></i>Get Started
                    </a>
                    <a class={format!("{} transition-all duration-200 ease-in-out", navbar_class(current_page == Page::Counter))}
                       href="#counter" on_click={nav(Page::Counter)}>
                        <i class="fas fa-code mr-2 opacity-70"></i>Examples
                    </a>
                    <div class="relative group">
                        <a class={navbar_class(current_page == Page::Concepts || current_page == Page::Signals || current_page == Page::Resources || current_page == Page::Effects)}
                           href="#concepts" on_click={nav(Page::Concepts)}>
                            Concepts
                            <i class="fas fa-chevron-down ml-1 text-xs"></i>
                        </a>
                        <div class="absolute left-0 mt-2 w-48 rounded-md shadow-lg bg-card border border-border hidden group-hover:block">
                            <div class="py-1">
                                <a class="block px-4 py-2 text-sm hover:bg-muted hover:text-primary-600" href="#concepts" on_click={nav(Page::Concepts)}>
                                    Overview
                                </a>
                                <a class="block px-4 py-2 text-sm hover:bg-muted hover:text-primary-600" href="#signals" on_click={nav(Page::Signals)}>
                                    Signals
                                </a>
                                <a class="block px-4 py-2 text-sm hover:bg-muted hover:text-primary-600" href="#resources" on_click={nav(Page::Resources)}>
                                    Resources
                                </a>
                                <a class="block px-4 py-2 text-sm hover:bg-muted hover:text-primary-600" href="#effects" on_click={nav(Page::Effects)}>
                                    Effects
                                </a>
                            </div>
                        </div>
                    </div>
                </nav>

                <div class="flex-1 flex justify-end">
                    <a
                        href="https://github.com/elcharitas/simple-rsx"
                        class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-background hover:bg-accent hover:text-accent-foreground h-9 py-2 px-3"
                        target="_blank"
                    >
                        <i class="fab fa-github mr-2"></i>
                        <span>GitHub</span>
                    </a>
                    <button type_="button" on_click={toggle_theme} class="inline-flex items-center justify-center rounded-full w-10 h-10 text-muted-foreground ml-2 hover:bg-muted/80 hover:text-primary transition-colors">
                        {either!(theme == "dark" => <i class="fas fa-sun text-lg transition-transform hover:rotate-45"></i> else <i class="fas fa-moon text-lg transition-transform hover:-rotate-12"></i>)}
                    </button>
                </div>
            </div>
            {children}
        </header>
    }
}

#[component]
pub fn Footer() -> Node {
    rsx! {
        <footer class="border-t border-border py-6 md:py-0">
            <div class="container flex flex-col items-center justify-between gap-4 md:h-14 md:flex-row">
                <p class="text-center text-sm leading-loose text-muted-foreground md:text-left">
                    Built with Momenta. Open source on " "
                    <a href="https://github.com/elcharitas/simple-rsx" class="font-medium underline underline-offset-4 hover:text-primary">GitHub</a>
                </p>
            </div>
        </footer>
    }
}

#[component]
pub fn FeatureTable() -> Node {
    rsx! {
        <div class="overflow-x-auto border border-border rounded-md mb-6">
            <table class="min-w-full divide-y divide-border">
                <thead class="bg-muted/50">
                    <tr>
                        <th class="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Feature</th>
                        <th class="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Description</th>
                        <th class="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Usage</th>
                    </tr>
                </thead>
                <tbody class="bg-card divide-y divide-border">
                    <TableRow
                        feature="wasm"
                        description="WebAssembly support for browser rendering"
                        default="simple-rsx = { version = \"0.1.0\", features = [\"wasm\"] }"
                    />
                    <TableRow
                        feature="signals"
                        description="Reactive state management with fine-grained updates"
                        default="simple-rsx = { version = \"0.1.0\", features = [\"signals\"] }"
                    />
                    <TableRow
                        feature="events"
                        description="DOM event handling support"
                        default="simple-rsx = { version = \"0.1.0\", features = [\"events\"] }"
                    />
                    <TableRow
                        feature="components"
                        description="Component-based architecture with props"
                        default="simple-rsx = { version = \"0.1.0\", features = [\"components\"] }"
                    />
                    <TableRow
                        feature="ssr"
                        description="Server-side rendering capabilities"
                        default="simple-rsx = { version = \"0.1.0\", features = [\"ssr\"] }"
                    />
                </tbody>
            </table>
        </div>
    }
}
