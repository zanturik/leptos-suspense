use leptos::prelude::*;
use leptos_meta::{Meta, provide_meta_context, MetaTags, Stylesheet};
use leptos_router::{
    path,
    components::{Route, Router, Routes, ParentRoute},
    StaticSegment,
    nested_router::Outlet
};
#[cfg(feature = "ssr")]
use tokio::time::{self, Duration};

#[server(GetData, "/api", "GetJson", "getData")]
async fn load_data(value: i32) -> Result<i32, ServerFnError> {
    dbg!(format!("executing load_data with {} value", &value));
    time::sleep(Duration::from_secs(value.try_into().unwrap())).await;
    Ok(value + 10)
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    leptos::logging::log!("App loaded...");
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-resources.css"/>
        <header>
            <nav>
                <a href="/">Home</a>
                <a href="/products/1/1">Product #1</a>
            </nav>
        </header>
        <div class="container">
        <Router>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("/") view=HomePage/>
                    <ParentRoute path=path!{"/products/:product_id"} view=ShowProduct>
                        <Route path=path!{":variant_id"} view=TestProduct/>
                    </ParentRoute>
                </Routes>
        </Router>
    </div>

    }
}

#[component]
fn HomePage() -> impl IntoView {
    let (count, set_count) = signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[component]
pub fn ShowProduct() -> impl IntoView {
    let (pr_id, _) = signal(3);
    let product = Resource::new_blocking(pr_id, |id| async move {
        load_data(id).await.unwrap_or_default()
    });

    view! {
        <Suspense>
            <Show when=move || product.get().is_some()>
                <Meta name="description" content=move || product.get().unwrap_or_default().to_string()/>
            </Show>
            <Outlet/>
        </Suspense>
    }
}

#[component]
pub fn TestProduct() -> impl IntoView {
    let (pr_id, _) = signal(2);
    let product = Resource::new_blocking(pr_id, |id| async move {
        load_data(id).await.unwrap_or_default()
    });

    view! {
        <Suspense>
            <Show when=move || product.get().is_some()>
                <Meta name="name" content=move || product.get().unwrap_or_default().to_string()/>
            </Show>
            <AnotherTestProduct/>
        </Suspense>
    }
}

#[component]
pub fn AnotherTestProduct() -> impl IntoView {
    let (pr_id, _) = signal(5);
    let product = Resource::new_blocking(pr_id, |id| async move {
        load_data(id).await.unwrap_or_default()
    });

    view! {
        <Suspense>
            <Show when=move || product.get().is_some()>
                <Meta name="comments" content=move || product.get().unwrap_or_default().to_string()/>
            </Show>
        </Suspense>
    }
}
