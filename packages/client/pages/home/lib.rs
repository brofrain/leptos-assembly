use client_components::{BaseButton, BaseInput, BaseLink, TheLogo};
use client_composables::confirm;
use client_env::PROJECT_REPOSITORY_URL;
use client_globals::prelude::*;
use client_i18n::{t_string, use_i18n};
use client_router::{use_navigate, HiParams, Route};
use client_stores::{use_store, Names};
use leptos_i18n::t;
use leptos_router::NavigateOptions;

#[component]
pub fn Index() -> impl IntoView {
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
            if confirm::show(
                confirm::Options::default()
                    .set_cancel(t!(i18n, home.confirm.cancel)),
            )
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
                <BaseLink to=PROJECT_REPOSITORY_URL>{t!(i18n, home.title)}</BaseLink>

                <p>
                    <em>{t!(i18n, home.description)}</em>
                </p>
            </div>

            <div class="py3"></div>

            <div>
                <BaseInput
                    value=name
                    placeholder=input_name_placeholder
                    on:keydown=handle_input_keydown
                />
            </div>

            <div>
                <BaseButton class="m3" on:click=move |_| navigate_name_hi() disabled=name_is_empty>
                    {t!(i18n, home.button)}
                </BaseButton>
            </div>
        </div>
    }
}
