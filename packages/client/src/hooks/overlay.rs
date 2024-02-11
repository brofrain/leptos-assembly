use client_utils::reactivity::{
    throttle::{self, UseSwitch},
    use_global_context_with_initializer,
};
use common::prelude::*;

#[derive(Clone, Copy)]
struct Ctx {
    switch: UseSwitch,
    show: RwSignal<bool>,
}

fn use_ctx() -> Ctx {
    use_global_context_with_initializer::<Ctx>(|| {
        let show = RwSignal::new(false);

        let switch = throttle::use_switch(
            move || show.set(true),
            move || show.set(false),
            10.milliseconds(),
        );

        Ctx { switch, show }
    })
}

pub fn use_switch() -> UseSwitch {
    use_ctx().switch
}

pub fn use_show() -> ReadSignal<bool> {
    use_ctx().show.read_only()
}
