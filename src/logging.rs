//! Setup for the application logging.
//!
//! - `Off`
//! - `Error`
//! - `Warn`
//! - `Info`
//! - `Debug`
//! - `Trace`
use log::{info, LevelFilter};
use std::sync::Once;

static INIT: Once = Once::new();

pub fn setup(level: log::LevelFilter) {
    if level == log::LevelFilter::Off {
        return;
    }

    INIT.call_once(|| {
        stdout_config(level);
    });
}

fn stdout_config(level: LevelFilter) {
    if let Err(_err) = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}][{}] {}",
                chrono::Local::now().format("%+"),
                record.target(),
                record.level(),
                message
            ));
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()
    {
        panic!("Failed to initialize logging.")
    }

    info!("logging initialized.");
}
