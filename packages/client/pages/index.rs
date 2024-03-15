use client_components::{BaseButton, BaseInput, BaseLink, TheLogo};
use client_env::PROJECT_REPOSITORY_URL;
use client_hooks::confirm;
use client_i18n::use_i18n;
use client_router::{use_navigate, HiParams, Route};
use client_stores::{use_store, Names};
use common::{prelude::*, vendor::web_sys};

#[component]
pub fn Index() -> impl IntoView {
    let i18n = use_i18n();
    let show_confirm = confirm::use_show();
    let navigate = use_navigate();

    let names_store = use_store::<Names>();

    let name = RwSignal::new(String::new());

    let input_name_placeholder = TextProp::from(move || {
        if let Some(last_name) = names_store.last_name() {
            t_string!(i18n, home.input_placeholder_with_name, name = last_name)
        } else {
            t_string!(i18n, home.input_placeholder)
        }
    });

    let name_is_empty = Signal::derive(move || with!(|name| name.is_empty()));

    let navigate_name_hi = move || {
        if name_is_empty() {
            return;
        }

        spawn_local(async move {
            if show_confirm(
                confirm::Options::default().set_cancel(t!(i18n, home.confirm.cancel)),
            )
            .await
            .is_accepted()
            {
                navigate(Route::Hi(HiParams::new(Some(name.get_untracked()))));
            }
        });
    };

    let handle_input_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" {
            navigate_name_hi();
        }
    };

    view! {
        <div class="text-center">
            <TheLogo class="text-5xl mb1"/>

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
