use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct JsonConn {
    pub name: String,

    pub addr: String,

    pub field: String,

    pub ok: String,
    pub warn: String,
    pub alert: String,
}
