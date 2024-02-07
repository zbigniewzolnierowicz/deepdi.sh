use yew::{function_component, html, use_state, ContextProvider, Html, Properties};

#[derive(Clone, Debug, PartialEq)]
pub struct ApiClient {
    base_url: String,
}

pub fn base_url() -> String {
    web_sys::window().unwrap().location().origin().unwrap()
}

impl ApiClient {
    pub fn new() -> Self {
        let base_url = base_url();
        ApiClient { base_url }
    }

    pub async fn fetch(&self) -> anyhow::Result<String> {
        let result = reqwest::get(self.base_url.clone() + "/api")
            .await?
            .text()
            .await?;

        Ok(result)
    }
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component]
pub fn ApiClientContextProvider(props: &Props) -> Html {
    let client = use_state(ApiClient::new);
    html! {
        <ContextProvider<ApiClient> context={(*client).clone()}>
            { props.children.clone() }
        </ContextProvider<ApiClient>>
    }
}
