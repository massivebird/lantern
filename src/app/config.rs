use super::connection::{Connection, json::JsonConn};
use serde::Deserialize;
use std::{io::Read, path::PathBuf};

#[derive(Clone, Debug, Deserialize)]
struct ConfigFile {
    connection: Vec<Connection>,
    json: Vec<JsonConn>,
}

pub fn read_config() -> Vec<Connection> {
    let home = std::env::var("HOME").unwrap();

    // Full path to the toml config file.
    let toml_path = PathBuf::from(home)
        .join(".config")
        .join("lantern")
        .join("config.toml");

    let Ok(mut f) = std::fs::File::open(&toml_path) else {
        eprintln!(
            "ERROR: failed to read configuration file: {}",
            toml_path.display()
        );
        std::process::exit(1);
    };

    let mut buf = String::new();

    if f.read_to_string(&mut buf).is_err() {
        eprintln!(
            "ERROR: failed to read configuration file: {}",
            toml_path.display()
        );
        std::process::exit(1);
    }

    let c: ConfigFile = toml::from_str(&buf).unwrap();

    [
        c.connection,
        c.json.into_iter().map(std::convert::Into::into).collect(),
    ]
    .concat()
}
