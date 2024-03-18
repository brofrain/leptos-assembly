#[cfg(target_arch = "wasm32")]
pub fn use_init() {
    use std::panic;

    use client_i18n::use_i18n;
    use common::prelude::*;

    use super::confirm;

    let i18n = use_i18n();

    panic::set_hook(Box::new(move |info| {
        let show_confirm = confirm::use_show();

        log::error!("{info}");

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

#[cfg(not(target_arch = "wasm32"))]
pub const fn use_init() {}
