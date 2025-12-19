use std::marker::PhantomData;

use clap::{Parser, Subcommand as ClapSubcommand};
use thiserror::Error;

use crate::{
    Config,
    cli::{ExecutableCommand, auto, generate_config, get},
};

#[derive(Debug, ClapSubcommand)]
#[command(version, about, long_about = None)]
pub enum Subcommand<'a> {
    Auto(auto::Command<'a>),
    Get(get::Command<'a>),
    GenerateConfig(generate_config::Command<'a>),
}

#[derive(Debug)]
pub struct Input<'config> {
    pub config: &'config Config,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to execute auto subcommand: {0}")]
    Auto(#[from] auto::Error),

    #[error("Failed to execute get subcommand: {0}")]
    Get(#[from] get::Error),

    #[error("Failed to execute generate-config subcommand: {0}")]
    GenerateConfig(#[from] generate_config::Error),
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
            Subcommand::Get(subcommand) => {
                let input = get::Input { config, reqwest };
                subcommand.execute(&input).await?;
            }
            Subcommand::GenerateConfig(subcommand) => {
                let input = generate_config::Input { config };
                subcommand.execute(&input).await?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_parse_auto_command() {
        let args = vec!["dnrs", "auto"];
        let command = Command::try_parse_from(args).unwrap();
        match command.subcommand {
            Subcommand::Auto(_) => (),
            _ => panic!("Expected Auto subcommand"),
        }
    }

    #[test]
    fn test_parse_get_command() {
        let args = vec!["dnrs", "get", "nitrado", "example.com"];
        let command = Command::try_parse_from(args).unwrap();
        match command.subcommand {
            Subcommand::Get(_) => (),
            _ => panic!("Expected Get subcommand"),
        }
    }

    #[test]
    fn test_parse_generate_config_command() {
        let args = vec!["dnrs", "generate-config"];
        let command = Command::try_parse_from(args).unwrap();
        match command.subcommand {
            Subcommand::GenerateConfig(_) => (),
            _ => panic!("Expected GenerateConfig subcommand"),
        }
    }
}
