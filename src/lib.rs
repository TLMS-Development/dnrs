pub mod config;
pub mod logger;

pub use config::{Config, EnvConfig, FileConfig, IpResolver, IpResolverType};
pub use logger::setup_logger;
use lum_log::info;

pub const TOKEN: &str = "nope";

pub async fn run(config: Config) {
    info!("Running with config: {:#?}", config);
}
