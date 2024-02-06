use common::prelude::*;

#[server]
pub async fn get() -> Result<String, ServerFnError> {
    use reqwest::{header::USER_AGENT, Client};

    #[derive(Deserialize, Clone)]
    struct Data {
        tag_name: String,
    }

    let tag = Client::new()
        .get("https://api.github.com/repos/leptos-rs/leptos/releases/latest")
        .header(USER_AGENT, "app")
        .send()
        .await?
        .json::<Data>()
        .await?
        .tag_name;

    Ok(tag)
}
