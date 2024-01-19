use client_components::BaseLink;
use client_i18n::use_i18n;
use exports::client::prelude::*;

#[component]
pub fn TheUpdateBanner() -> impl IntoView {
    let i18n = use_i18n();
    let show = RwSignal::new(false);

    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::{closure::Closure, JsCast};
        use wasm_bindgen_futures::JsFuture;

        spawn_local(async move {
            if let Ok(sw_ready_promise) =
                window().navigator().service_worker().ready()
            {
                if JsFuture::from(sw_ready_promise).await.is_ok() {
                    let controllerchange_cb =
                        Closure::<dyn Fn()>::wrap(Box::new(move || {
                            show.set(true);
                        }))
                        .into_js_value();

                    window()
                        .navigator()
                        .service_worker()
                        .set_oncontrollerchange(Some(
                            controllerchange_cb.unchecked_ref(),
                        ));
                }
            }
        });
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
