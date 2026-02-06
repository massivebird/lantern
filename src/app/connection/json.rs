use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct JsonConn {
    pub name: String,

    /// The API endpoint address.
    pub addr: String,

    /// The json field to test.
    ///
    /// E.g., `connections[4].status.name`
    pub field: String,

    pub ok: String,
    pub warn: String,
    pub alert: String,
}
