use std::panic;

use leptos::{spawn_local, window};

use super::confirm;
use crate::app::prelude::*;

pub fn init() {
    let i18n = use_i18n();

    panic::set_hook(Box::new(move |panic_info| {
        let msg = panic_info.to_string();
        log::error!("{msg}");

        spawn_local(async move {
            let body = t_view_untracked!(i18n, panic_confirm.body);
            let accept = t_view_untracked!(i18n, panic_confirm.accept);

            confirm::show(confirm::payload::Noncancelable { body, accept })
                .await;

            window().location().reload().unwrap();
        });
    }));
}
