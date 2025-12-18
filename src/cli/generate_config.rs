use std::{io, marker::PhantomData};

use clap::Parser;
use lum_log::info;
use thiserror::Error;

use crate::{Config, cli::ExecutableCommand};

#[derive(Debug)]
pub struct Input<'config> {
    pub config: &'config Config,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("YAML serialization error: {0}")]
    Yaml(#[from] serde_yaml_ng::Error),

    #[error("Config error: {0}")]
    Config(#[from] anyhow::Error),
}

/// Generate configuration directory structure
#[derive(Debug, Parser)]
#[command(version, about, long_about = None, propagate_version = true)]
pub struct Command<'command> {
    #[clap(skip)]
    _phantom: PhantomData<&'command ()>,

    /// Output directory path (defaults to ./config)
    #[clap(short, long, default_value = "config")]
    pub output: String,

    /// Force overwrite existing files
    #[clap(short, long, default_value = "false")]
    pub force: bool,
}

impl<'command> ExecutableCommand<'command> for Command<'command> {
    type I = Input<'command>;
    type R = Result<(), Error>;

    async fn execute(&self, _input: &'command Self::I) -> Self::R {
        let config_dir = std::path::Path::new(&self.output);

        if config_dir.exists() && !self.force {
            info!(
                "Configuration directory {:?} already exists. Use --force to overwrite.",
                config_dir
            );
            return Ok(());
        }

        Config::create_example_structure(config_dir)?;

        info!("Configuration structure created in {:?}", config_dir);

        Ok(())
    }
}
