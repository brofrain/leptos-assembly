use crate::app::{composables::nprogress, prelude::*};

#[server]
async fn get_leptos_tag() -> Result<String, ServerFnError> {
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

#[component]
pub fn TheLeptosTagInfo() -> impl IntoView {
    let i18n = use_i18n();

    let tag = Resource::local(
        || (),
        |()| async {
            nprogress::enable();
            let result = get_leptos_tag().await;
            nprogress::disable();
            result
        },
    );

    let msg = Memo::new(move |_| {
        tag().map_or_else(
            move || t_view!(i18n, home.leptos_tag_error),
            |tag| t_view!(i18n, home.leptos_tag, tag),
        )
    });

    view! {
        <div class=uno!["text-(sm center) italic", "op70", "mb2"]>
            <Suspense fallback=t!(i18n, home.leptos_tag_loading)>{msg}</Suspense>
        </div>
    }
}