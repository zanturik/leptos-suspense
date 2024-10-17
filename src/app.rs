use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

async fn load_data(value: i32) -> i32 {
    value + 10
}

#[component]
pub fn App() -> impl IntoView {
    logging::log!("App loaded...");
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-resources.css"/>
        <header>
            <nav>
                <a href="/">Home</a>
                <a href="/products/1">Product #1</a>
            </nav>
        </header>
        <div class="container">
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route ssr=SsrMode::PartiallyBlocked path="/products/:product_id" view=ShowProduct>
                    </Route>
                </Routes>
        </Router>
    </div>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[component]
pub fn ShowProduct() -> impl IntoView {
    let (pr_id, _) = create_signal(32);
    let product = create_blocking_resource(pr_id, |id| async move { load_data(id).await });

    view! {
        <Suspense>
            <Show when=move || product.get().map(|v| v.to_string()).is_some()>
            <Meta name="description" content=move || product.get().map(|v| v.to_string()).unwrap_or_default()/>
            </Show>
        </Suspense>
    }
}
