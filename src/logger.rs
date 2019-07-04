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

pub fn init_logger() -> Result<(), log::SetLoggerError> {
    log::set_logger(&LOGGER)?;
    log::set_max_level(log::LevelFilter::Debug);
    Ok(())
}

