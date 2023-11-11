use client_composables::panic_handler;
use client_i18n::provide_i18n_context;
use client_router::{HiParams, NotFoundParams, Route};
use client_utils::reactivity::provide_global_context;
use exports::client::prelude::*;
use leptos_meta::{provide_meta_context, Html, Link, Meta, Title};
use leptos_router::{Route as RouteView, Router, Routes};
use leptos_use::use_color_mode;
use macros::is_ssr;

flatten_mod!(the_confirms, the_overlay, the_toasts);

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_global_context();

    let i18n = provide_i18n_context();

    let head = view! {
        <Title text=t!(i18n, meta.title)/>
        <Link rel="apple-touch-icon" href="/pwa-192x192.png"/>
        <Link rel="mask-icon" href="/safari-pinned-tab.svg"/>
        <Meta name="msapplication-TileColor" content="#00aba9"/>

        <Link rel="stylesheet" href="/style.css"/>
        <Link rel="stylesheet" href="/webfonts.css"/>

        {#[cfg(feature = "pwa")]
        {
            use leptos_meta::Script;
            view! { <Script src="/registerSW.js"/> }
        }}

        <Html class="dark"/>
    };

    if is_ssr!() {
        return head;
    }

    panic_handler::init();
    use_color_mode();

    view! {
        {head}

        <div id="app">
            <Router>
                <Routes>
                    <RouteView path="" view=client_layouts::Home>
                        <RouteView path=Route::Home view=client_page_home::Index/>
                    </RouteView>

                    <RouteView path="" view=client_layouts::Default>
                        <RouteView
                            path=Route::Hi(HiParams {
                                name: ":name".to_owned(),
                            })

                            view=client_page_hi::Index
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
