use super::connection::{Connection, json::JsonConn};
use eyre::Context;
use serde::Deserialize;
use std::{io::Read, path::PathBuf};

#[derive(Clone, Debug, Deserialize)]
struct ConfigFile {
    connection: Vec<Connection>,
    json: Vec<JsonConn>,
}

pub fn read_config() -> eyre::Result<Vec<Connection>> {
    let home = std::env::var("HOME").wrap_err("Failed to read HOME environment variable")?;

    // Full path to the toml config file.
    let toml_path = PathBuf::from(home)
        .join(".config")
        .join("lantern")
        .join("config.toml");

    let mut f = std::fs::File::open(&toml_path)?;

    let mut buf = String::new();

    f.read_to_string(&mut buf)?;

    let c: ConfigFile = toml::from_str(&buf)?;

    Ok([
        c.connection,
        c.json.into_iter().map(std::convert::Into::into).collect(),
    ]
    .concat())
}
