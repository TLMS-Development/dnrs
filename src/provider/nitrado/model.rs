use core::num;
use std::{
    net::{self, Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

use lum_libs::serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::types::dns::{self, MxRecord, RecordType, RecordValue};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub enum RecordMode {
    #[serde(rename = "auto")]
    Auto,

    #[serde(rename = "manual")]
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct Record {
    pub r#type: RecordType,
    pub content: String,
    pub name: String,
    pub mode: RecordMode,
}

#[derive(Debug, Clone, Error)]
pub enum TryFromRecordError {
    #[error("Invalid IP address: {0}")]
    InvalidIp(#[from] net::AddrParseError),

    #[error("Invalid MX record format: {0}")]
    InvalidMxFormat(String),

    #[error("Invalid priority in MX record: {0}")]
    InvalidMxPriority(num::ParseIntError),

    #[error("Invalid SRV record format: {0}")]
    InvalidSrvFormat(String),

    #[error("Invalid SRV record priority/weight/port: {0}")]
    InvalidSrvValue(num::ParseIntError),

    #[error("Invalid TLSA record format: {0}")]
    InvalidTlsaFormat(String),

    #[error("Invalid TLSA record usage/selector/matching type: {0}")]
    InvalidTlsaValue(num::ParseIntError),

    #[error("Invalid CAA record format: {0}")]
    InvalidCaaFormat(String),

    #[error("Invalid CAA record flag: {0}")]
    InvalidCaaFlag(num::ParseIntError),

    #[error("Record type {0:?} is not supported by Nitrado provider")]
    UnsupportedRecordType(RecordType),
}

/// Converts a Nitrado API record into the internal [`dns::Record`] type.
///
/// # Examples
///
/// ```
/// use dnrs::provider::nitrado::model::{Record, RecordMode};
/// use dnrs::types::dns::{RecordType, RecordValue};
/// use std::convert::TryFrom;
///
/// let api_record = Record {
///     r#type: RecordType::A,
///     content: "1.2.3.4".to_string(),
///     name: "example.com".to_string(),
///     mode: RecordMode::Manual,
/// };
///
/// let dns_record = dnrs::types::dns::Record::try_from(api_record).unwrap();
/// assert_eq!(dns_record.domain, "example.com");
/// if let RecordValue::A(ip) = dns_record.value {
///     assert_eq!(ip.to_string(), "1.2.3.4");
/// } else {
///     panic!("Expected A record");
/// }
/// ```
impl TryFrom<Record> for dns::Record {
    type Error = TryFromRecordError;

    fn try_from(api_record: Record) -> Result<Self, Self::Error> {
        let value = match api_record.r#type {
            RecordType::A => {
                let ip = Ipv4Addr::from_str(&api_record.content)?;
                RecordValue::A(ip)
            }
            RecordType::AAAA => {
                let ip = Ipv6Addr::from_str(&api_record.content)?;
                RecordValue::AAAA(ip)
            }
            RecordType::CNAME => RecordValue::CNAME(api_record.content),
            RecordType::TXT => RecordValue::TXT(api_record.content),
            RecordType::SPF => RecordValue::SPF(api_record.content),
            RecordType::NS | RecordType::SOA => {
                return Err(TryFromRecordError::UnsupportedRecordType(api_record.r#type));
            }
            RecordType::MX => {
                let content = api_record.content;
                let parts: Vec<&str> = content.split_whitespace().collect();
                if parts.len() != 2 {
                    return Err(TryFromRecordError::InvalidMxFormat(content));
                }

                let priority = parts[0]
                    .parse::<u16>()
                    .map_err(TryFromRecordError::InvalidMxPriority)?;

                let target = parts[1].to_string();
                RecordValue::MX(MxRecord { priority, target })
            }
            RecordType::SRV => {
                let content = api_record.content;
                let parts: Vec<&str> = content.split_whitespace().collect();
                if parts.len() != 4 {
                    return Err(TryFromRecordError::InvalidSrvFormat(content));
                }

                let priority = parts[0]
                    .parse::<u16>()
                    .map_err(TryFromRecordError::InvalidSrvValue)?;
                let weight = parts[1]
                    .parse::<u16>()
                    .map_err(TryFromRecordError::InvalidSrvValue)?;
                let port = parts[2]
                    .parse::<u16>()
                    .map_err(TryFromRecordError::InvalidSrvValue)?;

                let target = parts[3].to_string();
                RecordValue::SRV(priority, weight, port, target)
            }
            RecordType::TLSA => {
                let content = api_record.content;
                let parts: Vec<&str> = content.split_whitespace().collect();
                if parts.len() != 4 {
                    return Err(TryFromRecordError::InvalidTlsaFormat(content));
                }

                let usage = parts[0]
                    .parse::<u16>()
                    .map_err(TryFromRecordError::InvalidTlsaValue)?;
                let selector = parts[1]
                    .parse::<u16>()
                    .map_err(TryFromRecordError::InvalidTlsaValue)?;
                let matching_type = parts[2]
                    .parse::<u16>()
                    .map_err(TryFromRecordError::InvalidTlsaValue)?;

                let cert_data = parts[3].to_string();
                RecordValue::TLSA(usage, selector, matching_type, cert_data)
            }
            RecordType::CAA => {
                let content = api_record.content;
                let parts: Vec<&str> = content.split_whitespace().collect();
                if parts.len() != 3 {
                    return Err(TryFromRecordError::InvalidCaaFormat(content));
                }

                let flag = parts[0]
                    .parse::<u8>()
                    .map_err(TryFromRecordError::InvalidCaaFlag)?;

                let tag = parts[1].to_string();
                let value = parts[2].to_string();
                RecordValue::CAA(flag, tag, value)
            }
        };

        Ok(dns::Record {
            domain: api_record.name,
            value,
            ttl: None, // Nitrado API does not provide TTL on GET
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct GetRecordsResponse {
    pub status: String,
    pub message: Vec<Record>,
}

impl TryFrom<GetRecordsResponse> for Vec<dns::Record> {
    type Error = TryFromRecordError;

    fn try_from(response: GetRecordsResponse) -> Result<Self, Self::Error> {
        response
            .message
            .into_iter()
            .map(dns::Record::try_from)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::dns::{RecordValue, RecordType};

    #[test]
    fn test_nitrado_record_to_dns_record_a() {
        let api_record = Record {
            r#type: RecordType::A,
            content: "1.2.3.4".to_string(),
            name: "example.com".to_string(),
            mode: RecordMode::Manual,
        };
        let dns_record = dns::Record::try_from(api_record).unwrap();
        assert_eq!(dns_record.domain, "example.com");
        match dns_record.value {
            RecordValue::A(ip) => assert_eq!(ip.to_string(), "1.2.3.4"),
            _ => panic!("Expected A record"),
        }
    }

    #[test]
    fn test_nitrado_record_to_dns_record_aaaa() {
        let api_record = Record {
            r#type: RecordType::AAAA,
            content: "::1".to_string(),
            name: "example.com".to_string(),
            mode: RecordMode::Manual,
        };
        let dns_record = dns::Record::try_from(api_record).unwrap();
        match dns_record.value {
            RecordValue::AAAA(ip) => assert_eq!(ip.to_string(), "::1"),
            _ => panic!("Expected AAAA record"),
        }
    }

    #[test]
    fn test_nitrado_record_to_dns_record_mx() {
        let api_record = Record {
            r#type: RecordType::MX,
            content: "10 mail.example.com".to_string(),
            name: "example.com".to_string(),
            mode: RecordMode::Manual,
        };
        let dns_record = dns::Record::try_from(api_record).unwrap();
        match dns_record.value {
            RecordValue::MX(mx) => {
                assert_eq!(mx.priority, 10);
                assert_eq!(mx.target, "mail.example.com");
            }
            _ => panic!("Expected MX record"),
        }
    }

    #[test]
    fn test_nitrado_record_to_dns_record_srv() {
        let api_record = Record {
            r#type: RecordType::SRV,
            content: "0 5 5060 sip.example.com".to_string(),
            name: "_sip._tcp.example.com".to_string(),
            mode: RecordMode::Manual,
        };
        let dns_record = dns::Record::try_from(api_record).unwrap();
        match dns_record.value {
            RecordValue::SRV(priority, weight, port, target) => {
                assert_eq!(priority, 0);
                assert_eq!(weight, 5);
                assert_eq!(port, 5060);
                assert_eq!(target, "sip.example.com");
            }
            _ => panic!("Expected SRV record"),
        }
    }

    #[test]
    fn test_nitrado_record_to_dns_record_invalid_ip() {
        let api_record = Record {
            r#type: RecordType::A,
            content: "invalid".to_string(),
            name: "example.com".to_string(),
            mode: RecordMode::Manual,
        };
        let result = dns::Record::try_from(api_record);
        assert!(result.is_err());
    }

    #[test]
    fn test_nitrado_record_to_dns_record_unsupported() {
        let api_record = Record {
            r#type: RecordType::NS,
            content: "ns1.example.com".to_string(),
            name: "example.com".to_string(),
            mode: RecordMode::Manual,
        };
        let result = dns::Record::try_from(api_record);
        assert!(matches!(result, Err(TryFromRecordError::UnsupportedRecordType(RecordType::NS))));
    }
}
