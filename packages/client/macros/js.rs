/// Extracts a JS function from the bindings file.
///
/// # Example
///
/// ```ignore
/// // top level function
/// bind_js_fn! { console_log }
/// bind_js_fn! { pub console_log }
///
/// // nested in an object
/// bind_js_fn! { nprogress => start }
/// bind_js_fn! { nprogress => pub start }
/// ```
#[macro_export]
macro_rules! bind_js_fn {
    (@wrap $js_namespace:ident => $($fn_definition:tt)*) => {
        #[$crate::__exports::wasm_bindgen::prelude::wasm_bindgen(
            raw_module = "/assets/bindings.js"
        )]
        extern "C" {
            #[allow(unsafe_code)]
            #[wasm_bindgen(js_namespace = $js_namespace)]
            $($fn_definition)*;
        }
    };

    (@wrap $($fn_definition:tt)*) => {
        #[$crate::__exports::wasm_bindgen::prelude::wasm_bindgen(
            raw_module = "/assets/bindings.js"
        )]
        extern "C" {
            #[allow(unsafe_code)]
            $($fn_definition)*;
        }
    };

    ($js_namespace:ident => pub $($fn_name:tt)*) => {
        bind_js_fn!(@wrap $js_namespace => pub fn $($fn_name)*());
    };

    ($js_namespace:ident => $($fn_name:tt)*) => {
        bind_js_fn!(@wrap $js_namespace => fn $($fn_name)*());
    };

    (pub $($fn_name:tt)*) => {
        bind_js_fn!(@wrap fn $($fn_name)*());
    };

    ($($fn_name:tt)*) => {
        bind_js_fn!(@wrap fn $($fn_name)*());
    };
}
