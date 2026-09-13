use lum_libs::serde::{Deserialize, Serialize};

use crate::provider::{hetzner, netcup, nitrado};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Provider {
    Nitrado(nitrado::Config),
    Hetzner(hetzner::Config),
    Netcup(netcup::Config),
}
