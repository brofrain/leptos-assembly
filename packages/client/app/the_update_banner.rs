use client_components::BaseLink;
use client_i18n::use_i18n;
use exports::client::prelude::*;

#[component]
pub fn TheUpdateBanner() -> impl IntoView {
    let i18n = use_i18n();
    let show = RwSignal::new(false);

    #[cfg(all(target_arch = "wasm32", feature = "pwa"))]
    {
        use wasm_bindgen::{closure::Closure, JsCast};
        use web_sys::ServiceWorkerState;

        let update_cb = Closure::<dyn Fn()>::wrap(Box::new(move || {
            show.set(true);
        }))
        .into_js_value();

        let sw_container = window().navigator().service_worker();

        if let Some(sw) = window().navigator().service_worker().controller()
            && sw.state() == ServiceWorkerState::Activated
        {
            sw_container
                .set_oncontrollerchange(Some(update_cb.unchecked_ref()));
        } else {
            let init_cb = Closure::<dyn Fn()>::wrap(Box::new({
                let sw_container = sw_container.clone();
                move || {
                    sw_container.set_oncontrollerchange(Some(
                        update_cb.unchecked_ref(),
                    ));
                }
            }))
            .into_js_value();

            sw_container.set_oncontrollerchange(Some(init_cb.unchecked_ref()));
        }
    }

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
