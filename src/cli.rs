pub mod auto;
pub mod command;
pub mod get;

use std::future::Future;

pub use command::{Command, Subcommand};

pub trait ExecutableCommand<'input> {
    type I;
    type R;

    fn execute(&self, input: &'input Self::I) -> impl Future<Output = Self::R> + Send;
}
