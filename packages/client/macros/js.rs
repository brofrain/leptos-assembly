// @kw fix binding path and remove the tmp bindings

/// Extracts a JS function from the bindings file.
///
/// # Example
///
/// ```ignore
/// // top level function
/// bind_js_fn! { console_log() }
///
/// // nested in an object
/// bind_js_fn! { nprogress => start() }
/// ```
#[macro_export]
macro_rules! bind_js_fn {
    ($js_namespace:ident => $($function_definition:tt)*) => {
        #[wasm_bindgen::prelude::wasm_bindgen(
            module = "bindings.mjs"
        )]
        extern "C" {
            #[allow(unsafe_code)]
            #[wasm_bindgen(js_namespace = $js_namespace)]
            fn $($function_definition)*;
        }
    };

    ($($function_definition:tt)*) => {
        #[wasm_bindgen::prelude::wasm_bindgen(
            module = "bindings.mjs"
        )]
        extern "C" {
            #[allow(unsafe_code)]
            fn $($function_definition)*;
        }
    };
}
