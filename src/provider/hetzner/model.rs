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
pub struct Record {
    pub r#type: RecordType,
    pub id: String,
    pub created: String,
    pub modified: String,
    pub zone_id: String,
    pub name: String,
    pub value: String,
    pub ttl: Option<u32>,
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
}

impl TryFrom<Record> for dns::Record {
    type Error = TryFromRecordError;

    fn try_from(api_record: Record) -> std::result::Result<Self, Self::Error> {
        let value = match api_record.r#type {
            RecordType::A => {
                let ip = Ipv4Addr::from_str(&api_record.value)?;
                RecordValue::A(ip)
            }
            RecordType::AAAA => {
                let ip = Ipv6Addr::from_str(&api_record.value)?;
                RecordValue::AAAA(ip)
            }
            RecordType::CNAME => RecordValue::CNAME(api_record.value),
            RecordType::TXT => RecordValue::TXT(api_record.value),
            RecordType::SPF => RecordValue::SPF(api_record.value),
            RecordType::NS => RecordValue::NS(api_record.value),
            RecordType::SOA => RecordValue::SOA(api_record.value),
            RecordType::MX => {
                let content = api_record.value;
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
                let content = api_record.value;
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
                let content = api_record.value;
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
                let content = api_record.value;
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
            ttl: api_record.ttl,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct GetRecordsResponse {
    pub records: Vec<Record>,
}

impl TryFrom<GetRecordsResponse> for Vec<dns::Record> {
    type Error = TryFromRecordError;

    fn try_from(response: GetRecordsResponse) -> std::result::Result<Self, Self::Error> {
        response
            .records
            .into_iter()
            .map(dns::Record::try_from)
            .collect()
    }
}
