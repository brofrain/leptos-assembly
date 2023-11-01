#![feature(lazy_cell)]

use cfg_if::cfg_if;
use common_macros::{cfg_csr, cfg_ssr};
use fern::Dispatch;

#[cfg(feature = "ssr")]
fn build_server_dispatch(dispatch: Dispatch) -> Dispatch {
    use owo_colors::OwoColorize;
    use time::{macros::format_description, OffsetDateTime};

    let time_format =
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

    dispatch
        .chain(std::io::stdout())
        .format(|out, message, record| {
            let level = record.level();

            macro_rules! bold_string {
                    ($($t:tt)*) => {
                        $($t)*.bold().to_string()
                    };
                }

            let level = match level {
                log::Level::Error => bold_string!(level.red()),
                log::Level::Warn => bold_string!(level.yellow()),
                log::Level::Info => bold_string!(level.blue()),
                log::Level::Debug => bold_string!(level.bright_magenta()),
                log::Level::Trace => bold_string!(level.magenta()),
            };

            let module =
                record.module_path().unwrap_or_else(|| record.target());
            let module = module.green();
            let module = module.dimmed();

            let path = record.file().map(|path| {
                let line = record.line();
                match line {
                    Some(line) => format!("{path}:{line}"),
                    None => path.to_owned(),
                }
            });

            let module = match path {
                Some(path) => {
                    format!("{module} {}", path.dimmed())
                }
                None => module.to_string(),
            };

            let time = OffsetDateTime::now_utc()
                .format(time_format)
                .unwrap_or_else(|_| "Unknown time".to_owned());
            let time = time.cyan();
            let time = time.dimmed();

            out.finish(format_args!("{level} {time} {module} {message}",));
        })
}

pub fn init(app_crate_name: &'static str) {
    let mut dispatch = Dispatch::new();

    // debug / info logs from external crates don't interest us
    dispatch = dispatch.level(log::LevelFilter::Warn);

    cfg_if! {
        if #[cfg(debug_assertions)] {
            dispatch = dispatch.level_for(app_crate_name, log::LevelFilter::Debug);
        } else {
            dispatch = dispatch.level_for(app_crate_name, log::LevelFilter::Info);
        }
    }

    cfg_csr! {
        dispatch = dispatch.chain(fern::Output::call(console_log::log)).format(
            |out, message, _| {
                out.finish(*message);
            },
        );
    }

    cfg_ssr! {
        dispatch = build_server_dispatch(dispatch);
    }

    dispatch.apply().expect("Failed to initialize logger");
}
