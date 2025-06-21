use std::marker::PhantomData;

use clap::{Parser, Subcommand as ClapSubcommand};
use lum_log::error;
use thiserror::Error;

use crate::{
    Config,
    cli::{ExecutableCommand, auto},
};

#[derive(Debug, ClapSubcommand)]
#[command(version, about, long_about = None, propagate_version = true)]
pub enum Subcommand<'a> {
    Auto(auto::Command<'a>),
}

#[derive(Debug)]
pub struct Input<'config> {
    pub config: &'config Config,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to execute auto subcommand: {0}")]
    Auto(#[from] auto::Error),
}

/// dnrs
#[derive(Debug, Parser)]
#[command(version, about, long_about = None, propagate_version = true)]
pub struct Command<'command> {
    #[clap(skip)]
    _phantom: PhantomData<&'command ()>,
    /*
    TODO: Implement when supported by lum_log
    /// Show verbose output
    #[clap(short, long, default_value = "false")]
    pub verbose: bool,
    */
    #[command(subcommand)]
    pub subcommand: Subcommand<'command>,
}

impl<'command> ExecutableCommand<'command> for Command<'command> {
    type I = Input<'command>;
    type R = Result<(), Error>;

    async fn execute(&self, input: &'command Self::I) -> Self::R {
        let config = input.config;
        let reqwest = reqwest::Client::new();

        match &self.subcommand {
            Subcommand::Auto(subcommand) => {
                let input = auto::Input { config, reqwest };
                subcommand.execute(&input).await?;
            }
        }

        Ok(())
    }
}
