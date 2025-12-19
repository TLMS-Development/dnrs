use std::net::{Ipv4Addr, Ipv6Addr};

use lum_libs::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct MxRecord {
    pub priority: u16,
    pub target: String,
}

/// Represents the value of a DNS record.
///
/// # Examples
///
/// ```
/// use dnrs::types::dns::{RecordValue, MxRecord};
/// use std::net::Ipv4Addr;
///
/// let a_record = RecordValue::A(Ipv4Addr::new(127, 0, 0, 1));
/// let mx_record = RecordValue::MX(MxRecord { priority: 10, target: "mail.example.com".to_string() });
///
/// assert!(matches!(a_record, RecordValue::A(_)));
/// assert!(matches!(mx_record, RecordValue::MX(_)));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub enum RecordValue {
    A(Ipv4Addr),
    AAAA(Ipv6Addr),
    CNAME(String),
    TXT(String),
    SPF(String),
    MX(MxRecord),
    NS(String),
    SOA(String),
    SRV(u16, u16, u16, String),
    TLSA(u16, u16, u16, String),
    CAA(u8, String, String),
}

/// Represents a DNS record.
///
/// # Examples
///
/// ```
/// use dnrs::types::dns::{Record, RecordValue};
/// use std::net::Ipv4Addr;
///
/// let record = Record {
///     domain: "example.com".to_string(),
///     value: RecordValue::A(Ipv4Addr::new(1, 2, 3, 4)),
///     ttl: Some(3600),
/// };
///
/// assert_eq!(record.domain, "example.com");
/// assert!(matches!(record.value, RecordValue::A(_)));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct Record {
    pub domain: String,
    pub value: RecordValue,
    pub ttl: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub enum RecordType {
    A,
    AAAA,
    CNAME,
    TXT,
    SPF,
    MX,
    NS,
    SOA,
    SRV,
    TLSA,
    CAA,
}
