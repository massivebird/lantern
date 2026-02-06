use serde::{Deserialize, Deserializer};
use std::{borrow::Cow, collections::VecDeque};

mod address;
pub mod json;
mod status;

pub use address::Address;
pub use status::Status;

pub const MAX_STATUSES: usize = 30;

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
    pub fn push_status(&mut self, code: Status) {
        if self.log.len() == MAX_STATUSES {
            self.log.pop_back();
        }

        self.log.push_front(code);
    }

    pub const fn log(&self) -> &VecDeque<Status> {
        &self.log
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
