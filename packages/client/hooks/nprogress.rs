use client_macros::bind_js_fn;
use client_utils::reactivity::{
    throttle::{self, UseSwitch},
    use_global_context_with_initializer,
};
use common::vendor::time::ext::NumericalDuration;

bind_js_fn! { nprogress => fn start() }
bind_js_fn! { nprogress => fn done() }

#[derive(Clone, Copy)]
struct Ctx {
    switch: UseSwitch,
}

fn use_ctx() -> Ctx {
    use_global_context_with_initializer::<Ctx>(|| {
        let switch = throttle::use_switch(start, done, 100.milliseconds());
        Ctx { switch }
    })
}

pub fn use_switch() -> UseSwitch {
    use_ctx().switch
}
