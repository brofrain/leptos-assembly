use client_macros::bind_js_fn;
use client_utils::reactivity::{
    throttle::{use_switch, UseSwitch},
    use_global_context_with_initializer,
};
use exports::time::ext::NumericalDuration;

bind_js_fn! { nprogress => start() }
bind_js_fn! { nprogress => done() }

#[derive(Clone, Copy)]
struct Ctx {
    switch: UseSwitch,
}

fn use_ctx() -> Ctx {
    use_global_context_with_initializer::<Ctx>(|| {
        let switch = use_switch(start, done, 100.milliseconds());
        Ctx { switch }
    })
}

pub fn enable() {
    use_ctx().switch.enable();
}

pub fn disable() {
    use_ctx().switch.disable();
}
