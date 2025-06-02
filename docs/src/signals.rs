use alloc::vec;
use simple_rsx::{Node, component, rsx};

use crate::components::*;

#[component]
pub fn SignalsPage() -> Node {
    use alloc::vec;

    rsx! {
        <section class="py-16 bg-background">
            <div class="container">
                <div class="max-w-3xl mx-auto">
                    <PageHeader
                        title="Signals"
                        subtitle="Reactive state management through fine-grained reactivity"
                    />

                    <div class="prose max-w-none">
                        <h2 class="text-2xl font-bold mb-4">What are Signals?</h2>
                        <p class="mb-4">
                            Signals are the core primitive for reactive state management in Momenta.
                            They are containers for values that can change over time, automatically
                            notifying any components that depend on them to re-render.
                        </p>

                        <InfoBox variant="info" title="Fine-Grained Reactivity" content="Signals only update the components that depend on them, making your UI fast and efficient." icon="fas fa-bolt">
                            <p>
                                Signals provide fine-grained reactivity, meaning only the components
                                that actually use a signal will re-render when it changes, not the
                                entire component tree.
                            </p>
                        </InfoBox>

                        <h2 class="text-2xl font-bold mb-4 mt-8">Creating Signals</h2>
                        <p class="mb-4">
                            The <code>create_signal</code> function is used to create a new signal with an initial value.
                            It returns a signal object that can be used to get and set the value.
                        </p>

                        <CodeBlock
                            code="let count = create_signal(0);\n\n// Reading the value\nlet current = count.get();\n\n// Setting a new value\ncount.set(current + 1);"
                            language="rust"
                            title="Creating a Signal"
                        />

                        <h2 class="text-2xl font-bold mb-4 mt-8">Signal Operations</h2>
                        <p class="mb-4">
                            Signals provide several methods for working with their values:
                        </p>

                        <TableRow
                            feature="get()"
                            description="Retrieves the current value of the signal. This creates a dependency tracking relationship."
                            default="loading"
                        />

                        <TableRow
                            feature="set(value)"
                            description="Updates the signal with a new value and triggers re-renders of dependent components."
                            default="loading"
                        />

                        <TableRow
                            feature="with(fn)"
                            description="Executes a function with the current value, useful for complex transformations without cloning."
                            default="loading"
                        />

                        <h2 class="text-2xl font-bold mb-4 mt-8">Derived Signals</h2>
                        <p class="mb-4">
                            You can create derived signals that depend on other signals. These will automatically
                            update when their dependencies change.
                        </p>

                        <CodeBlock
                            code="let count = create_signal(0);\n\n// Create a derived computation\ncreate_effect(move || {\n    let doubled = count.get() * 2;\n    println!(\"Doubled value: {}\", doubled);\n});"
                            language="rust"
                            title="Derived Signals"
                        />

                        <h2 class="text-2xl font-bold mb-4 mt-8">Signal Implementation Details</h2>
                        <p class="mb-4">
                            Under the hood, signals use a dependency tracking system to keep track of which components
                            depend on which signals. When a signal&apos;s value changes, it notifies all dependent components
                            to re-render.
                        </p>

                        <InfoBox variant="warning" title="Scope Requirement" content="Signals must be created inside a component or reactive scope to work properly." icon="fas fa-exclamation-circle">
                            <p>
                                Signals must be created within a reactive scope (like a component). Creating signals
                                outside of a scope will result in an error.
                            </p>
                        </InfoBox>

                        <h2 class="text-2xl font-bold mb-4 mt-8">Best Practices</h2>
                        <ul class="list-disc pl-6 mb-6">
                            <li>Keep signal values immutable when possible</li>
                            <li>Use the smallest possible state granularity</li>
                            <li>Avoid creating signals in loops or frequently called functions</li>
                            <li>Use derived signals for computed values instead of storing derived state</li>
                        </ul>

                        <div class="flex justify-between mt-12">
                            <a href="#concepts" class="text-primary-600 hover:underline">"← Back to Concepts"</a>
                            <a href="#resources" class="text-primary-600 hover:underline">"Resources →"</a>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
