use self::json::JsonConn;
use serde::{Deserialize, Deserializer};
use std::{borrow::Cow, collections::VecDeque};

mod address;
pub mod json;
mod status;

pub use address::Address;
pub use status::Status;

pub const MAX_STATUSES: usize = 40;

#[derive(Clone, Debug, Deserialize)]
pub struct Connection {
    pub name: String,

    #[serde(rename = "addr")]
    #[serde(deserialize_with = "deserialize_conn")]
    pub addr: Address,

    #[serde(skip)]
    log: VecDeque<Status>,
}

impl Connection {
    pub fn push_status(&mut self, status: Status) {
        if self.log.len() == MAX_STATUSES {
            self.log.pop_back();
        }

        let status = status.id(self
            .log
            .front()
            .map(|st| st.id.wrapping_add(1))
            .unwrap_or_default());

        self.log.push_front(status);
    }

    pub const fn log(&self) -> &VecDeque<Status> {
        &self.log
    }

    pub fn pretty_name(&self) -> String {
        let symbol = match &self.addr {
            Address::Remote { .. } => '󰖟',
            Address::Local { .. } => '󰇅',
            Address::Json { .. } => '󱂛',
        };

        format!("{symbol} {}", self.name)
    }

    /// Returns the effective address.
    pub fn addr(&self) -> Cow<'_, str> {
        match &self.addr {
            Address::Remote { url } | Address::Json { url, .. } => Cow::Borrowed(url),
            Address::Local { ip } => Cow::Owned(ip.to_string()),
        }
    }
}

fn deserialize_conn<'de, D>(deserializer: D) -> Result<Address, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = Cow::<'de, str>::deserialize(deserializer)?;

    Ok(buf.parse().map_or_else(
        |_| Address::Remote {
            url: buf.to_string(),
        },
        |ip| Address::Local { ip },
    ))
}

impl From<JsonConn> for Connection {
    fn from(value: JsonConn) -> Self {
        let addr = Address::Json {
            url: value.addr,
            field: value.field,
            ok: value.ok,
            warn: value.warn,
            alert: value.alert,
        };

        Self {
            name: value.name,
            addr,
            log: VecDeque::new(),
        }
    }
}
