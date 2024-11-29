use leptos::prelude::*;
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    StaticSegment,
};

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
                <HydrationScripts options islands=true/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[server(GetData, "/api", "GetJson", "getData")]
pub async fn get_data() -> Result<String, ServerFnError> {
    let pool = use_context::<FakePool>().ok_or(ServerFnError::new("context not found"))?;
    Ok(format!("This is the data # {}", pool.id))
}

#[component]
pub fn App() -> impl IntoView {
    let data = Resource::new_blocking(
        || (),
        move |_| async move { get_data().await.unwrap_or_else(|e| e.to_string()) },
    );
    provide_context(data);

    view! {
        <Router>
            <HeaderComponent/>
            <main style="background-color: lightblue; padding: 10px">
            <FlatRoutes fallback=|| "Not found.">
                <Route path=StaticSegment("") view=HomePage/>
            </FlatRoutes>
            </main>
        </Router>
    }
}

#[component]
fn HeaderComponent() -> impl IntoView {
    let context_data = use_context::<Resource<String>>().expect("Resource context not found!");

    let data = Suspend::new(async move {
        let super_string = context_data.await;
        view! {
            <b>{ super_string }</b>
        }
    });

    view! {
        <h1>This is the header</h1>
        <b>{ data }</b>
        <h3>Did you see the data?</h3>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h4>"Welcome to Leptos!"</h4>
    }
}
