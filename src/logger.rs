struct GameLogger;

impl log::Log for GameLogger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        eprintln!("{}", record.args());
    }

    fn flush(&self) {}
}

static LOGGER: GameLogger = GameLogger;

pub fn init_logger(debug_enabled: bool) -> Result<(), log::SetLoggerError> {
    log::set_logger(&LOGGER)?;

    let log_level = if debug_enabled {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    log::set_max_level(log_level);
    Ok(())
}
