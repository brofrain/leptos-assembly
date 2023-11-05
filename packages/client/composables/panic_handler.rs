use std::panic;

use client_i18n::use_i18n;
use common::prelude::*;

use super::confirm;

pub fn init() {
    let i18n = use_i18n();

    panic::set_hook(Box::new(move |panic_info| {
        let msg = panic_info.to_string();
        log::error!("{msg}");

        spawn_local(async move {
            confirm::show(
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
