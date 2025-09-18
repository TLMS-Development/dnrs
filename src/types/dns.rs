use std::net::{Ipv4Addr, Ipv6Addr};

use lum_libs::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct MxRecord {
    pub priority: u16,
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub enum RecordValue {
    A(Ipv4Addr),
    AAAA(Ipv6Addr),
    CNAME(String),
    TXT(String),
    MX(MxRecord),
    Custom(String, String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct Record {
    pub domain: String,
    pub value: RecordValue,
    pub ttl: Option<u32>,
}
