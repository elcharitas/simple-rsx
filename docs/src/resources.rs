use alloc::vec;
use simple_rsx::{Node, component, rsx};

use crate::components::*;

#[component]
pub fn ResourcesPage() -> Node {
    rsx! {
        <section class="py-16 bg-background">
            <div class="container">
                <div class="max-w-3xl mx-auto">
                    <PageHeader
                        title="Resources"
                        subtitle="Handling async data fetching with reactive resources"
                    />

                    <div class="prose max-w-none">
                        <h2 class="text-2xl font-bold mb-4">What are Resources?</h2>
                        <p class="mb-4">
                            Resources are a special type of reactive primitive designed for handling asynchronous
                            data fetching and operations. They combine signals with async operations to provide
                            a clean way to work with external data sources.
                        </p>

                        <InfoBox variant="info" title="Automatic State Tracking" content="Resources manage loading, error, and ready states for you, so your UI can reactively display the right content at the right time." icon="fas fa-sync-alt">
                            <p>
                                Resources automatically track loading states, errors, and data, making it easy
                                to build UIs that respond to the different states of async operations.
                            </p>
                        </InfoBox>

                        <h2 class="text-2xl font-bold mb-4 mt-8">Creating Resources</h2>
                        <p class="mb-4">
                            The <code>create_resource</code> function creates a new resource that fetches data
                            asynchronously. It takes a source signal (optional) and an async fetcher function.
                        </p>

                        <CodeBlock
                            code="// Basic resource without dependencies\nlet users = create_resource(\n    // Async fetcher function\n    async move || {\n        // Fetch data from an API\n        fetch(\"/api/users\").await.json().await\n    }\n);\n\n// Resource with a dependency\nlet user_id = create_signal(1);\nlet user_details = create_resource(\n    // Source signal\n    user_id,\n    // Async fetcher that depends on user_id\n    async move |id| {\n        fetch(&format!(\"/api/users/{}\", id)).await.json().await\n    }\n);"
                            language="rust"
                            title="Creating a Resource"
                        />

                        <h2 class="text-2xl font-bold mb-4 mt-8">Resource States</h2>
                        <p class="mb-4">
                            Resources have different states that you can check and respond to in your UI:
                        </p>

                        <TableRow
                            default="loading"
                            description="The initial state when a resource is fetching data."
                            feature="loading"
                        />

                        <TableRow
                            default="ready"
                            description="The resource has successfully fetched data and it&apos;s available."
                            feature="loading"
                        />

                        <TableRow
                            default="error"
                            description="An error occurred during the fetch operation."
                            feature="loading"
                        />

                        <h2 class="text-2xl font-bold mb-4 mt-8">Using Resource Data</h2>
                        <p class="mb-4">
                            You can access the current state and data of a resource in your components:
                        </p>

                        <CodeBlock
                            code="// In your component\nrsx! {\n    <div>\n        {match user_details.state() {\n            ResourceState::Loading => rsx!(<div>\"Loading...\"</div>),\n            ResourceState::Ready(data) => rsx!(\n                <div>\n                    <h1>{data.name}</h1>\n                    <p>{data.email}</p>\n                </div>\n            ),\n            ResourceState::Error(err) => rsx!(<div>\"Error: {err}\"</div>),\n        }}\n    </div>\n}"
                            language="rust"
                            title="Using Resource Data"
                        />

                        <h2 class="text-2xl font-bold mb-4 mt-8">Refetching Data</h2>
                        <p class="mb-4">
                            Resources automatically refetch when their dependencies change. You can also manually
                            trigger a refetch:
                        </p>

                        <CodeBlock
                            code="// Manually refetch data\nlet refetch = move |_| {\n    user_details.refetch();\n};\n\nrsx! {\n    <button on_click={refetch}>\"Refresh\"</button>\n}"
                            language="rust"
                            title="Refetching Data"
                        />

                        <h2 class="text-2xl font-bold mb-4 mt-8">Resource Implementation Details</h2>
                        <p class="mb-4">
                            Resources are built on top of signals and effects. When you create a resource, it sets up
                            an effect that watches the source signal and triggers the fetcher function when it changes.
                        </p>

                        <InfoBox variant="info" title="Scope Requirement" content="Resources must be created inside a component or reactive scope to work properly." icon="fas fa-exclamation-circle">
                            <p>
                                Resources must be created within a reactive scope (like a component). Creating resources
                                outside of a scope will result in an error.
                            </p>
                        </InfoBox>

                        <h2 class="text-2xl font-bold mb-4 mt-8">Best Practices</h2>
                        <ul class="list-disc pl-6 mb-6">
                            <li>Always handle all resource states in your UI (loading, ready, error)</li>
                            <li>Use resources for any async operation, not just data fetching</li>
                            <li>Keep fetcher functions pure and focused on data retrieval</li>
                            <li>Consider adding timeouts or retry logic for production applications</li>
                        </ul>

                        <div class="flex justify-between mt-12">
                            <a href="#signals" class="text-primary-600 hover:underline">"← Signals"</a>
                            <a href="#effects" class="text-primary-600 hover:underline">"Effects →"</a>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
