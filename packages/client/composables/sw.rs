
mod js {
    use client_macros::bind_js_fn;
    bind_js_fn! { sw => pub register }
}

pub fn register() {
    js::register();
}
