use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub enum ConnectionType {
    Remote { url: String },
    Local { ip: std::net::IpAddr },
}

impl Default for ConnectionType {
    fn default() -> Self {
        Self::Local {
            ip: std::net::Ipv4Addr::LOCALHOST.into(),
        }
    }
}
