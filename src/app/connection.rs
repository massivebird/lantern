use std::collections::VecDeque;

pub const MAX_STATUSES: usize = 50;

#[derive(Clone)]
pub struct Connection {
    pub name: String,
    pub conn_type: ConnectionType,
    log: VecDeque<Result<u16, ()>>,
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
            log: VecDeque::with_capacity(MAX_STATUSES),
            conn_type: ConnectionType::Web {
                url: addr.to_string(),
            },
        }
    }

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
