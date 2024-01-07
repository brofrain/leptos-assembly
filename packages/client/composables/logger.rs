pub fn init() {
    #[cfg(target_arch = "wasm32")]
    fern::Dispatch::new()
        .level(
            #[cfg(debug_assertions)]
            log::LevelFilter::Debug,
            #[cfg(not(debug_assertions))]
            log::LevelFilter::Warn,
        )
        .chain(fern::Output::call(console_log::log))
        .format(|out, message, _| {
            out.finish(*message);
        })
        .apply()
        .expect("Failed to initialize logger");
}
