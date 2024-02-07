pub mod components;
pub mod context;
pub mod routes;
pub mod utils;

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::utils::api::{ApiClient, ApiClientContextProvider};

#[function_component]
fn ButtonFetch() -> Html {
    let client = use_context::<ApiClient>();

    if client.is_none() {
        return html! {
            <div>{"No HTML client context!"}</div>
        };
    }

    let onclick = {
        let client = client.clone().unwrap();
        move |_| {
            let client = client.clone();
            spawn_local(async move {
                let body = client.fetch().await.unwrap();
                tracing::info!(body);
            })
        }
    };
    html! {
        <button onclick={onclick}>{"EEE"}</button>
    }
}

#[function_component]
#[tracing::instrument(name = "App")]
fn App() -> Html {
    html! {
        <ApiClientContextProvider>
            <ButtonFetch />
        </ApiClientContextProvider>
    }
}

pub fn render() {
    yew::Renderer::<App>::new().render();
}
