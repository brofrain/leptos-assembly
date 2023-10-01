use leptos_router::NavigateOptions;

use crate::{
    app::{
        components::{
            base::{Button, Input, Link},
            pages::HiParams,
            shared::TheLogo,
        },
        composables::confirm,
        prelude::*,
        router::{use_navigate, Route},
        stores::{use_store, Names},
    },
    env::APP_REPOSITORY_URL,
};

#[component]
pub fn Home() -> impl IntoView {
    let i18n = use_i18n();

    let names_store = use_store::<Names>();

    let name = RwSignal::new(String::new());

    let input_name_placeholder = (move || {
        if let Some(last_name) = names_store.last_name() {
            t_string!(i18n, home.input_placeholder_with_name, name = last_name)
        } else {
            t_string!(i18n, home.input_placeholder)
        }
    })
    .into_signal();

    let navigate_name_hi = move || {
        let name = name();

        let navigate = use_navigate();
        spawn_local(async move {
            if confirm::show(confirm::payload::Cancelable::default())
                .await
                .is_accepted()
            {
                navigate(
                    &Route::Hi(HiParams { name }),
                    NavigateOptions::default(),
                );
            }
        });
    };

    let handle_input_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" {
            navigate_name_hi();
        }
    };

    let name_is_empty = (move || name().is_empty()).into_signal();

    view! {
        <div class="text-center">
            <TheLogo class="inline-block text-5xl mb1"/>

            <div>
                <Link to=APP_REPOSITORY_URL>{t!(i18n, home.title)}</Link>

                <p>
                    <em>{t!(i18n, home.description)}</em>
                </p>
            </div>

            <div class="py3"></div>

            <div>
                <Input
                    value=name
                    placeholder=input_name_placeholder
                    on:keydown=handle_input_keydown
                />
            </div>

            <div>
                <Button class="m3" on:click=move |_| navigate_name_hi() disabled=name_is_empty>
                    {t!(i18n, home.button)}
                </Button>
            </div>
        </div>
    }
}
