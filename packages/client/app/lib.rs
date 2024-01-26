use client_hooks::{logger, panic_handler};
use client_i18n::provide_i18n_context;
use client_router::{HiParams, NotFoundParams, Route};
use client_utils::reactivity::provide_global_context;
use exports::client::prelude::*;
use leptos_meta::{provide_meta_context, Html, Link, Meta, Title};
use leptos_router::{Route as RouteView, Router, Routes};
use leptos_use::use_color_mode;

flatten_mod!(the_confirms, the_overlay, the_toasts, the_update_banner);

#[component]
pub fn App() -> impl IntoView {
    let i18n = provide_i18n_context();

    logger::use_init();
    panic_handler::use_init();
    provide_meta_context();
    provide_global_context();
    use_color_mode();

    #[cfg(all(target_arch = "wasm32", feature = "pwa"))]
    {
        use client_hooks::sw;
        sw::register();
    }

    view! {
        <Title text=t!(i18n, meta.title)/>
        <Meta name="description" content=t!(i18n, meta.description)/>
        <Meta name="msapplication-TileColor" content="#ffffff"/>
        <Meta name="theme-color" content="#ffffff"/>
        <Link rel="icon" href="/assets/favicon.ico"/>
        <Link rel="apple-touch-icon" href="/assets/pwa-192x192.png" sizes="192x192"/>
        <Link rel="mask-icon" href="/assets/safari-pinned-tab.svg"/>

        <Link rel="stylesheet" href="/assets/style.css"/>
        <Link rel="stylesheet" href="/assets/webfonts.css"/>

        {#[cfg(feature = "pwa")]
        {
            view! { <Link rel="manifest" href="/assets/manifest.webmanifest"/> }
        }}

        <Html class="dark"/>

        <div id="app">
            <Router>
                <Routes>
                    <RouteView path="" view=client_layouts::Home>
                        <RouteView path=Route::Home view=client_pages::Index/>
                    </RouteView>

                    <RouteView path="" view=client_layouts::Default>
                        <RouteView path=Route::Hi(None) view=client_pages::Hi/>
                        <RouteView
                            path=Route::Hi(
                                Some(HiParams {
                                    name: ":name".to_owned(),
                                }),
                            )

                            view=client_pages::HiName
                        />
                        <RouteView path=Route::About view=client_pages::About/>
                    </RouteView>

                    <RouteView path="" view=client_layouts::Blank>
                        <RouteView
                            path=Route::NotFound(NotFoundParams {
                                path: "*path".to_owned(),
                            })

                            view=client_pages::NotFound
                        />
                    </RouteView>

                </Routes>
            </Router>

            <TheUpdateBanner/>
            <TheOverlay/>
            <TheConfirms/>
            <TheToasts/>
        </div>
    }
}
