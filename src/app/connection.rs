use serde::{Deserialize, Deserializer};
use std::borrow::Cow;
use std::collections::VecDeque;

pub const MAX_STATUSES: usize = 50;

#[derive(Clone, Debug, Deserialize)]
pub struct Connection {
    pub name: String,

    #[serde(rename = "addr")]
    #[serde(deserialize_with = "deserialize_conn")]
    pub conn_type: ConnectionType,

    #[serde(skip)]
    log: VecDeque<Result<u16, ()>>,
}

pub(super) fn deserialize_conn<'de, D>(deserializer: D) -> Result<ConnectionType, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = Cow::<'de, str>::deserialize(deserializer)?;

    Ok(buf.parse().map_or_else(
        |_| ConnectionType::Web {
            url: buf.to_string(),
        },
        |ip| ConnectionType::Local { ip },
    ))
}

#[derive(Clone, Debug, Deserialize)]
pub enum ConnectionType {
    Web { url: String },
    Local { ip: std::net::IpAddr },
}

impl Default for ConnectionType {
    fn default() -> Self {
        Self::Local {
            ip: std::net::Ipv4Addr::LOCALHOST.into(),
        }
    }
}

impl Connection {
    pub fn push_status_code(&mut self, code: Result<u16, ()>) {
        if self.log.len() == MAX_STATUSES {
            self.log.pop_back();
        }

        self.log.push_front(code);
    }

    pub fn log(&self) -> VecDeque<Result<u16, ()>> {
        self.log.clone()
    }

    pub fn addr(&self) -> String {
        match &self.conn_type {
            ConnectionType::Web { url } => url.clone(),
            ConnectionType::Local { ip } => ip.to_string(),
        }
    }
}
