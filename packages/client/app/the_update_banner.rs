use client_components::BaseLink;
use client_hooks::sw;
use client_i18n::use_i18n;
use common::vendor::client::prelude::*;

#[component]
pub fn TheUpdateBanner() -> impl IntoView {
    let i18n = use_i18n();
    let show = sw::use_update_available();

    view! {
        <Show when=show>
            <div class=uno![
                "absolute", "inset-(t-0 x-0)", "select-none", "p1", "bg-accent",
                "text-(center accent-contrast)", "flex-center", "flex-wrap", "gap-x2"
            ]>
                {t!(i18n, sw_update.body)}
                <BaseLink
                    on:click=move |_| {
                        window().location().reload().unwrap();
                    }

                    class="!text-accent-contrast"
                >
                    {t!(i18n, sw_update.accept)}
                </BaseLink>
            </div>
        </Show>
    }
}
