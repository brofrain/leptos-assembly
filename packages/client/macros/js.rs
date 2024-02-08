/// Extracts a JS function from the bindings file.
///
/// # Example
///
/// ```ignore
/// // top level function
/// bind_js_fn! { fn console_log(msg: &str) }
/// bind_js_fn! { pub fn console_log(msg: &str) }
///
/// // nested in an object
/// bind_js_fn! { nprogress => fn start() }
/// bind_js_fn! { nprogress => pub fn start() }
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

    ($js_namespace:ident => pub fn $($fn_name_and_types:tt)*) => {
        bind_js_fn!(@wrap $js_namespace => pub fn $($fn_name_and_types)*);
    };

    ($js_namespace:ident => fn $($fn_name_and_types:tt)*) => {
        bind_js_fn!(@wrap $js_namespace => fn $($fn_name_and_types)*);
    };

    (pub fn $($fn_name_and_types:tt)*) => {
        bind_js_fn!(@wrap pub fn $($fn_name_and_types)*);
    };

    (fn $($fn_name_and_types:tt)*) => {
        bind_js_fn!(@wrap fn $($fn_name_and_types)*);
    };
}
