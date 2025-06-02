use alloc::vec;
use simple_rsx::{Node, component, rsx};

use crate::components::*;

#[component]
pub fn EffectsPage() -> Node {
    rsx! {
        <section class="py-16 bg-background">
            <div class="container">
                <div class="max-w-3xl mx-auto">
                    <PageHeader
                        title="Effects"
                        subtitle="Managing side effects and reactive computations"
                    />

                    <div class="prose max-w-none">
                        <h2 class="text-2xl font-bold mb-4">What are Effects?</h2>
                        <p class="mb-4">
                            Effects are functions that run when their dependencies change. They&apos;re used for
                            handling side effects and computations that depend on reactive state, such as:
                        </p>
                        <ul class="list-disc pl-6 mb-6">
                            <li>Updating the DOM directly</li>
                            <li>Making API calls in response to state changes</li>
                            <li>Setting up and cleaning up subscriptions</li>
                            <li>Logging or analytics</li>
                            <li>Syncing with external systems</li>
                        </ul>

                        <InfoBox variant="info" title="Automatic Dependency Tracking" content="Effects re-run automatically when any of their dependencies change, keeping your UI in sync." icon="fas fa-sync-alt">
                            <p>
                                Effects automatically track their dependencies. When you access a signal inside
                                an effect, it will re-run whenever that signal changes.
                            </p>
                        </InfoBox>

                        <h2 class="text-2xl font-bold mb-4 mt-8">Creating Effects</h2>
                        <p class="mb-4">
                            The <code>create_effect</code> function creates a new effect that runs immediately
                            and then re-runs whenever its dependencies change.
                        </p>

                        <CodeBlock
                            code="let count = create_signal(0);\n\n// Create an effect that depends on count\ncreate_effect(move || {\n    let current = count.get();\n    println!(\"Count changed to: {}\", current);\n    \n    // This effect will re-run whenever count changes\n});"
                            language="rust"
                            title="Creating an Effect"
                        />

                        <h2 class="text-2xl font-bold mb-4 mt-8">Effect Dependencies</h2>
                        <p class="mb-4">
                            Effects automatically track which signals they depend on. When you call <code>signal.get()</code>
                            inside an effect, it establishes a dependency relationship.
                        </p>

                        <CodeBlock
                            code="let first_name = create_signal(\"John\");\nlet last_name = create_signal(\"Doe\");\n\n// This effect depends on both first_name and last_name\ncreate_effect(move || {\n    let full_name = format!(\"{} {}\", first_name.get(), last_name.get());\n    println!(\"Name changed to: {}\", full_name);\n});\n\n// This will trigger the effect\nfirst_name.set(\"Jane\");\n\n// This will also trigger the effect\nlast_name.set(\"Smith\");"
                            language="rust"
                            title="Effect Dependencies"
                        />

                        <h2 class="text-2xl font-bold mb-4 mt-8">Common Use Cases</h2>
                        <p class="mb-4">
                            Here are some common patterns for using effects:
                        </p>

                        <h3 class="text-xl font-bold mb-2">Syncing with localStorage</h3>
                        <CodeBlock
                            code="let theme = create_signal(\"light\");\n\n// Load from localStorage on initialization\ncreate_effect(move || {\n    // This runs once on initialization\n    if let Some(saved_theme) = localStorage.get_item(\"theme\") {\n        theme.set(saved_theme);\n    }\n});\n\n// Save to localStorage when theme changes\ncreate_effect(move || {\n    let current_theme = theme.get();\n    localStorage.set_item(\"theme\", current_theme);\n});"
                            language="rust"
                            title="Syncing with localStorage"
                        />

                        <h3 class="text-xl font-bold mb-2 mt-4">DOM Manipulation</h3>
                        <CodeBlock
                            code="let scroll_position = create_signal(0);\n\n// Update scroll position when it changes\ncreate_effect(move || {\n    let pos = scroll_position.get();\n    window.scroll_to(0, pos);\n});"
                            language="rust"
                            title="DOM Manipulation"
                        />

                        <h2 class="text-2xl font-bold mb-4 mt-8">Effect Cleanup</h2>
                        <p class="mb-4">
                            Effects are automatically cleaned up when their parent scope is destroyed. This prevents
                            memory leaks and ensures that subscriptions are properly terminated.
                        </p>

                        <InfoBox variant="warning" title="Scope Requirement" content="Effects must be created inside a component or reactive scope to work properly." icon="fas fa-exclamation-circle">
                            <p>
                                Effects must be created within a reactive scope (like a component). Creating effects
                                outside of a scope will result in an error.
                            </p>
                        </InfoBox>

                        <h2 class="text-2xl font-bold mb-4 mt-8">Best Practices</h2>
                        <ul class="list-disc pl-6 mb-6">
                            <li>Keep effects focused on a single responsibility</li>
                            <li>Avoid creating signals inside effects</li>
                            <li>Be careful with effects that update signals they depend on (can cause infinite loops)</li>
                            <li>Use effects for side effects, not for deriving state (use computed values instead)</li>
                        </ul>

                        <div class="flex justify-between mt-12">
                            <a href="#resources" class="text-primary-600 hover:underline">"← Resources"</a>
                            <a href="#concepts" class="text-primary-600 hover:underline">Back to Concepts</a>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
