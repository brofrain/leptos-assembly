use client_composables::panic_handler;
use client_i18n::provide_i18n_context;
use client_router::{HiParams, NotFoundParams, Route};
use client_utils::reactivity::provide_global_context;
use exports::client::prelude::*;
use leptos_meta::{provide_meta_context, Html, Link, Meta, Title};
use leptos_router::{Route as RouteView, Router, Routes};
use leptos_use::use_color_mode;

flatten_mod!(the_confirms, the_overlay, the_toasts);

#[component]
pub fn App() -> impl IntoView {
    let i18n = provide_i18n_context();

    panic_handler::init();
    provide_meta_context();
    provide_global_context();
    use_color_mode();

    view! {
        <Title text=t!(i18n, meta.title)/>
        <Link rel="apple-touch-icon" href="/assets/pwa-192x192.png"/>
        <Link rel="mask-icon" href="/assets/safari-pinned-tab.svg"/>
        <Meta name="msapplication-TileColor" content="#00aba9"/>

        <Link rel="stylesheet" href="/assets/style.css"/>
        <Link rel="stylesheet" href="/assets/webfonts.css"/>

        {#[cfg(feature = "pwa")]
        {
            use leptos_meta::Script;
            view! { <Script src="/assets/registerSW.js"/> }
        }}

        <Html class="dark"/>

        <div id="app">
            <Router>
                <Routes>
                    <RouteView path="" view=client_layouts::Home>
                        <RouteView path=Route::Home view=client_page_home::Index/>
                    </RouteView>

                    <RouteView path="" view=client_layouts::Default>
                        <RouteView path=Route::Hi(None) view=client_page_hi_index::Index/>
                        <RouteView
                            path=Route::Hi(
                                Some(HiParams {
                                    name: ":name".to_owned(),
                                }),
                            )

                            view=client_page_hi_name::Index
                        />
                        <RouteView path=Route::About view=client_page_about::Index/>
                    </RouteView>

                    <RouteView path="" view=client_layouts::Blank>
                        <RouteView
                            path=Route::NotFound(NotFoundParams {
                                path: "*path".to_owned(),
                            })

                            view=client_page_404::Index
                        />
                    </RouteView>

                </Routes>
            </Router>

            <TheOverlay/>
            <TheToasts/>
            <TheConfirms/>
        </div>
    }
}
