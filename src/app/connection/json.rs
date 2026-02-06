use serde::Deserialize;
use std::borrow::Cow;
use std::collections::VecDeque;

use super::{Address, Connection, Status};

#[derive(Clone, Debug, Deserialize)]
pub struct JsonConn {
    pub name: String,

    pub addr: String,

    field: String,

    ok: String,
    warn: String,
    alert: String,
}

impl Into<Connection> for JsonConn {
    fn into(self) -> Connection {
        let addr = Address::Json {
            url: self.addr,
            field: self.field,
            ok: self.ok,
            warn: self.warn,
            alert: self.alert,
        };

        Connection {
            name: self.name,
            addr,
            log: VecDeque::new(),
        }
    }
}
