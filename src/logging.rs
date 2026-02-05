use std::fs::OpenOptions;
use colored::Colorize;
use log::{Level, LevelFilter};
use fern::{Dispatch, Output};

pub(crate) fn setup_logger() -> Result<(), fern::InitError> {
    let log_level = LevelFilter::Info;

    Dispatch::new()
        .format(|out, message, record| {
            let colored_level = match record.level() {
                Level::Error => "ERROR".red(),
                Level::Warn => "WARN".yellow(),
                Level::Info => "INFO".white(),
                Level::Debug => "DEBUG".dimmed(),
                Level::Trace => "TRACE".white(),
            };
            out.finish(format_args!(
                "[{} {} @ {}:{}] {}",
                // humantime::format_rfc3339_seconds(SystemTime::now()),
                colored_level,
                record.target(),
                record.file().unwrap_or("n/a"),
                record.line().unwrap_or(0),
                message
            ));
        })
        .level(log_level)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}