use clap::Parser;
use lum_log::debug;
use std::time::Instant;
use thiserror::Error;

use crate::cli::{Command, ExecutableCommand, command::Input};

pub mod cli;
pub mod config;
pub mod logger;
pub mod provider;
pub mod resolver;
pub mod types;

pub use config::{Config, EnvConfig, FileConfig};
pub use logger::setup_logger;

pub const TOKEN: &str = "nope";
pub const PROGRAM_NAME: &str = env!("CARGO_PKG_NAME");
pub const PROGRAM_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Error while executing command: {0}")]
    Command(#[from] cli::command::Error),
}

pub async fn run(config: Config) -> Result<(), RuntimeError> {
    let start = Instant::now();

    let command = Command::parse();
    let input = Input { config: &config };
    command.execute(&input).await?;

    let elapsed = start.elapsed();
    debug!("Done in {}ms", elapsed.as_millis());

    Ok(())
}
