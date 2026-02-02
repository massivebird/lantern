use super::connection::Connection;
use serde::Deserialize;
use std::{io::Read, path::PathBuf};

#[derive(Clone, Debug, Deserialize)]
struct ConfigFile {
    connection: Vec<Connection>,
}

pub fn read_config() -> Vec<Connection> {
    let home = std::env::var("HOME").unwrap();

    // Full path to the toml config file.
    let toml_path = PathBuf::from(format!("{home}/.config/lanturn/config.toml"));

    let mut f = std::fs::File::open(&toml_path).unwrap();

    let mut buf = String::new();
    f.read_to_string(&mut buf)
        .expect("Failed to read contents of TOML config file.");

    let c: ConfigFile = toml::from_str(&buf).unwrap();

    c.connection
}
