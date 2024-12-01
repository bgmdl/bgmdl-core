use colored::Colorize;
use log::LevelFilter;

pub fn get_color(level: log::Level) -> colored::Color {
    match level {
        log::Level::Trace => colored::Color::White,
        log::Level::Debug => colored::Color::Blue,
        log::Level::Info => colored::Color::Green,
        log::Level::Warn => colored::Color::Yellow,
        log::Level::Error => colored::Color::Red,
    }
}

pub fn setup_logger(level: LevelFilter) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string().as_str().magenta(),
                record.level().as_str().color(get_color(record.level())),
                record.target().color(colored::Color::Cyan),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}
