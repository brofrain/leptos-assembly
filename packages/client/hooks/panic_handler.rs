use std::panic;

use client_i18n::use_i18n;
use common::vendor::client::prelude::*;

use super::confirm;

pub fn use_init() {
    let i18n = use_i18n();

    panic::set_hook(Box::new(move |panic_info| {
        let show_confirm = confirm::use_show();

        let msg = panic_info.to_string();
        log::error!("{msg}");

        spawn_local(async move {
            show_confirm(
                confirm::Options::default()
                    .set_body(t!(i18n, panic_confirm.body))
                    .set_accept(t!(i18n, panic_confirm.accept))
                    .disable_cancel(),
            )
            .await;

            window().location().reload().unwrap();
        });
    }));
}