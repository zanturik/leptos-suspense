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
                <a href="/products/1/2">Product #1</a>
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
                    <Route path="/products/:product_id" view=ShowProduct>
                        <Route path=":id" view=ShowVariant/>
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
pub fn ShowVariant() -> impl IntoView {
    let (var_id, _) = create_signal(55);
    let variant = create_resource(var_id, |id| async move { load_data(id).await });

    view! {
        <h3>Refresh this page (CTRL+F5)</h3>
        {
            move || match variant.get() {
                None => view! { <p>"no variant found... yet..."</p> }.into_view(),
                Some(s) => view! { <p>variant! { s }</p>
                }.into_view()
            }
        }
    }
}

#[component]
pub fn ShowProduct() -> impl IntoView {
    let (pr_id, _) = create_signal(32);
    let product = create_blocking_resource(pr_id, |id| async move { load_data(id).await });

    view! {

        <Suspense
        fallback=move || view! { <p> empty </p>}
    >
    {
        move || product.get().map(|pr|
            view! {
                <h1>{ pr }</h1>
                <Outlet/>
            }
        )
    }
            </Suspense>
    }
}
