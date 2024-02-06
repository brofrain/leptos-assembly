use common::{prelude::*, vendor::web_sys::ServiceWorkerState};
use wasm_bindgen::{closure::Closure, JsCast};

mod js {
    use client_macros::bind_js_fn;
    bind_js_fn! { sw => pub register }
}

#[derive(Clone, Copy)]
struct UpdateAvailable(ReadSignal<bool>);

pub fn register() {
    js::register();

    let update_available = RwSignal::new(false);
    provide_context(UpdateAvailable(update_available.read_only()));

    let update_cb = Closure::<dyn Fn()>::wrap(Box::new(move || {
        update_available.set(true);
    }))
    .into_js_value();

    let sw_container = window().navigator().service_worker();

    if let Some(sw) = window().navigator().service_worker().controller()
        && sw.state() == ServiceWorkerState::Activated
    {
        sw_container.set_oncontrollerchange(Some(update_cb.unchecked_ref()));
    } else {
        let init_cb = Closure::<dyn Fn()>::wrap(Box::new({
            let sw_container = sw_container.clone();
            move || {
                sw_container
                    .set_oncontrollerchange(Some(update_cb.unchecked_ref()));
            }
        }))
        .into_js_value();

        sw_container.set_oncontrollerchange(Some(init_cb.unchecked_ref()));
    }
}

pub fn use_update_available() -> ReadSignal<bool> {
    use_context::<UpdateAvailable>()
        .map_or_else(|| RwSignal::new(false).read_only(), |v| v.0)
}
