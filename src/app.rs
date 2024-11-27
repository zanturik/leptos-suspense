use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos_axum::ResponseOptions;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use reactive_stores::Store;
use serde::{Deserialize, Serialize};
#[derive(Clone)]
pub struct FakePool {
    pub id: i32,
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

#[derive(Debug, Store, Serialize, Deserialize, Default)]
pub struct StoreData {
    product: Option<String>,
}

#[component]
pub fn App() -> impl IntoView {
    // leptos::logging::log!("App loaded...");
    provide_meta_context();

    let store = Store::new(StoreData::default());
    provide_context(store);

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
                    <Route path=StaticSegment("/") view=ShowProduct/>
                </Routes>
        </Router>
    </div>

    }
}

#[component]
pub fn ShowProduct() -> impl IntoView {
    let store =
        use_context::<Store<StoreData>>().expect("App is not loaded. Store context not found!");

    let product = Resource::new_blocking(
        || (),
        move |_| async move { String::from("You shell no pass!") },
    );

    let show_product = Suspend::new(async move {
        let prod = product.await;
        store.product().set(Some(prod.clone()));
        view! { <h1>"Unsere reihe:" { prod }</h1>}
    });

    view! {

        { show_product }
        <b>Main component</b>
        <TestProduct/>

    }
}

#[component]
pub fn TestProduct() -> impl IntoView {
    let store =
        use_context::<Store<StoreData>>().expect("App is not loaded. Store context not found!");

    let variant = Resource::new_blocking(
        move || store.product().get(),
        |p| async move {
            match p {
                Some(s) => get_variants(s).await.unwrap_or_default(),
                None => String::from("No variant No cry!"),
            }
        },
    );

    view! {
        <Suspense>
        { variant }
        <b>Just a test</b>
         </Suspense>
    }
}

#[server(GetVariants, "/api", "GetJson", "getVariants")]
pub async fn get_variants(sl: String) -> Result<String, ServerFnError> {
    let _db_pool = use_context::<FakePool>().ok_or(ServerFnError::new("Cannot get db pool"))?;
    let _response = expect_context::<ResponseOptions>();
    let _db_pool2 = use_context::<FakePool>().ok_or(ServerFnError::new("Cannot get db pool"))?;
    let _response3 = expect_context::<ResponseOptions>();
    let _db_pool4 = use_context::<FakePool>().ok_or(ServerFnError::new("Cannot get db pool"))?;
    let _response5 = expect_context::<ResponseOptions>();
    let _db_pool6 = use_context::<FakePool>().ok_or(ServerFnError::new("Cannot get db pool"))?;
    let _response7 = expect_context::<ResponseOptions>();
    let _db_pool8 = use_context::<FakePool>().ok_or(ServerFnError::new("Cannot get db pool"))?;
    let _response9 = expect_context::<ResponseOptions>();
    let _db_pool10 = use_context::<FakePool>().ok_or(ServerFnError::new("Cannot get db pool"))?;
    let _response11 = expect_context::<ResponseOptions>();

    Ok(sl + " bazinga!")
}
