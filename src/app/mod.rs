mod composables;
mod router;
mod stores;

mod components {
    mod base;
    mod shared;
    mod utility;

    pub mod layouts;
    pub mod pages;

    mod root {
        flatten_pub_mod!(the_overlay);
        flatten_pub_mod!(the_toasts);
        flatten_pub_mod!(the_confirms);
    }
    pub use root::*;
}

mod prelude;

use components::{TheConfirms, TheOverlay, TheToasts};
use leptos_meta::{provide_meta_context, Html, Link, Meta, Title};
use leptos_router::Router;
use leptos_use::use_color_mode;

use crate::app::{
    composables::{
        i18n::provide_i18n_context,
        panic_handler,
        provide_global_context,
    },
    prelude::*,
};

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

        // PWA should be enabled only in production
        {#[cfg(not(debug_assertions))]
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
                <router::View></router::View>
            </Router>

            <TheOverlay/>
            <TheToasts/>
            <TheConfirms/>
        </div>
    }
}
