use common::prelude::*;
use client_utils::reactivity::use_global_context_with_initializer;

use super::throttle::{use_switch, UseSwitch};

#[derive(Clone, Copy)]
struct Ctx {
    switch: UseSwitch,
    show: RwSignal<bool>,
}

fn use_ctx() -> Ctx {
    use_global_context_with_initializer::<Ctx>(|| {
        let show = RwSignal::new(false);

        let switch = use_switch(
            move || show.set(true),
            move || show.set(false),
            10.milliseconds(),
        );

        Ctx { switch, show }
    })
}

pub fn enable() {
    use_ctx().switch.enable();
}

pub fn disable() {
    use_ctx().switch.disable();
}

pub fn use_show() -> ReadSignal<bool> {
    use_ctx().show.read_only()
}
