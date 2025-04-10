use std::io::Write;

use env_logger::Env;
use log::Level;

/// Custom logger
///
/// Default env: RUST_LOG=info
/// INFO - Just stdout
/// OTHER - regular eng_logger
pub fn init() {
    let mut builder = env_logger::Builder::from_env(Env::default().default_filter_or("info"));
    builder
        .format(|buf, record| {
            // Format INFO level messages without any prefix/metadata
            if record.level() == Level::Info {
                writeln!(buf, "{}", record.args())
            } else {
                // Use the default colored formatter for non-INFO messages
                let level = record.level();
                let mut level_style = buf.style();

                match level {
                    Level::Trace => level_style.set_color(env_logger::fmt::Color::Magenta),
                    Level::Debug => level_style.set_color(env_logger::fmt::Color::Cyan),
                    Level::Info => level_style.set_color(env_logger::fmt::Color::Green),
                    Level::Warn => level_style.set_color(env_logger::fmt::Color::Yellow),
                    Level::Error => level_style
                        .set_color(env_logger::fmt::Color::Red)
                        .set_bold(true),
                };

                writeln!(
                    buf,
                    "[{}] {}: {}",
                    level_style.value(level),
                    record.target(),
                    record.args()
                )
            }
        })
        .init();
}
