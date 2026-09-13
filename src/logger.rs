use lum_log::{
    ConfigBuilder, ConfigBuilderError,
    log::{LevelFilter::Info, SetLoggerError},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SetupLoggerError {
    #[error("Config builder error: {0}")]
    ConfigBuilderError(#[from] ConfigBuilderError),

    #[error("Set logger error: {0}")]
    SetLoggerError(#[from] SetLoggerError),
}

pub fn setup_logger() -> Result<(), SetupLoggerError> {
    let config = ConfigBuilder::default()
        .root_log_level(Info)
        .stdout_console_appender()
        .build()?;

    lum_log::setup(config)?;
    Ok(())
}
