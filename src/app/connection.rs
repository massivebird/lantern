use std::collections::VecDeque;

pub const MAX_STATUSES: usize = 50;

#[derive(Clone)]
pub struct Connection {
    pub name: String,
    pub conn_type: ConnectionType,
    status_codes: VecDeque<Option<Result<u16, ()>>>,
}

#[derive(Clone, Debug)]
pub enum ConnectionType {
    Web { url: String },
    Local { ip: std::net::IpAddr },
}

impl Connection {
    pub(super) fn new(name: &str, addr: &str) -> Self {
        Self {
            name: name.to_string(),
            status_codes: vec![None; MAX_STATUSES].into(),
            conn_type: ConnectionType::Web {
                url: addr.to_string(),
            },
        }
    }

    pub fn push_status_code(&mut self, code: Option<Result<u16, ()>>) {
        if self.status_codes.len() == MAX_STATUSES {
            self.status_codes.pop_back();
        }

        self.status_codes.push_front(code);
    }

    pub fn get_status_codes(&self) -> VecDeque<Option<Result<u16, ()>>> {
        self.status_codes.clone()
    }

    pub fn addr(&self) -> String {
        match &self.conn_type {
            ConnectionType::Web { url } => url.clone(),
            ConnectionType::Local { ip } => ip.to_string(),
        }
    }
}
