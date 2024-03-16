use client_components::{BaseIcon, BaseLink};
use client_env::PROJECT_REPOSITORY_URL;
use client_hooks::i18n;
use client_i18n::use_i18n;
use client_router::Route;
use common::{
    prelude::*,
    vendor::leptos_use::{use_color_mode, ColorMode, UseColorModeReturn},
};

#[component]
pub fn Footer() -> impl IntoView {
    let i18n = use_i18n();

    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode();

    let toggle_dark = move |_| {
        set_mode(if mode() == ColorMode::Dark {
            ColorMode::Light
        } else {
            ColorMode::Dark
        });
    };

    let toggle_locale = {
        let toggle = i18n::use_toggle_locale_and_push_toast();
        move |_| toggle()
    };

    view! {
        <nav class=uno!["flex", "justify-center", "gap-3", "text-xl", "my-4", "children:lh-0"]>
            <BaseLink title=TextProp::from(t!(i18n, nav.link_home)) to=Route::Home>
                <BaseIcon icon=icon::AiHomeOutlined/>
            </BaseLink>

            <BaseLink title=TextProp::from(t!(i18n, nav.toggle_dark)) on:click=toggle_dark>
                <BaseIcon icon=icon::ChSun class="dark:hidden"/>
                <BaseIcon icon=icon::TbMoon class="hidden dark:block"/>
            </BaseLink>

            <BaseLink title=TextProp::from(t!(i18n, nav.toggle_locale)) on:click=toggle_locale>
                <BaseIcon icon=icon::IoLanguage/>
            </BaseLink>

            <BaseLink title=TextProp::from(t!(i18n, nav.link_about)) to=Route::About>
                <BaseIcon icon=icon::BsCardText/>
            </BaseLink>

            <BaseLink title="GitHub" to=PROJECT_REPOSITORY_URL>
                <BaseIcon icon=icon::AiGithubFilled/>
            </BaseLink>
        </nav>
    }
}
