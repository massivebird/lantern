use self::{
    cli::generate_matches, connection::Connection, output_fmt::OutputFmt, selected_tab::SelectedTab,
};
use std::{
    path::Path,
    sync::{Arc, Mutex},
};
use yaml_rust2::Yaml;

pub mod cli;
pub mod connection;
pub mod output_fmt;
pub mod selected_tab;

#[derive(Default)]
pub struct App {
    pub connections: Arc<Mutex<Vec<Connection>>>,
    pub output_fmt: OutputFmt,
    pub selected_tab: SelectedTab,
    log_conn_idx: usize,
    is_closing: bool,
}

impl App {
    pub fn generate() -> Self {
        let matches: clap::ArgMatches = generate_matches();

        let conns = Self::read_config();

        let output_fmt = match matches.get_one::<OutputFmt>("output_fmt") {
            Some(&fmt) => fmt,
            None => OutputFmt::default(),
        };

        Self {
            connections: Arc::new(Mutex::new(conns)),
            output_fmt,
            ..Default::default()
        }
    }

    fn read_config() -> Vec<Connection> {
        let home_dir = std::env::var("HOME").unwrap();
        let config_path = format!("{home_dir}/.config/lanturn/config.yaml");

        assert!(
            Path::new(&config_path).exists(),
            "Unable to locate config file at {config_path}",
        );

        let Ok(config_contents) = std::fs::read_to_string(config_path.clone()) else {
            panic!("Unable to read config file at {config_path}");
        };

        let Ok(yaml) = yaml_rust2::YamlLoader::load_from_str(&config_contents) else {
            panic!("Failed to parse config file at {config_path} into yaml.")
        };

        let sites_yaml: &Yaml = &yaml[0]["sites"];

        let mut sites: Vec<Connection> = Vec::new();

        // I don't know how to iterate over yaml::as_hash() without
        // unwrapping it, and that panics when unwrapping zero users.
        // So if there are no users, we exit this block.
        if sites_yaml.as_hash().is_none() {
            unimplemented!();
        }

        for (label, properties) in sites_yaml.as_hash().unwrap() {
            let Some(label) = label.as_str() else {
                panic!("Failed to process label: {label:?}");
            };

            let Some(name) = properties["name"].as_str() else {
                panic!("Failed to process field `name` for user labeled `{label}`");
            };

            let Some(url) = properties["url"].as_str() else {
                panic!("Failed to process field `url` for user labeled `{label}`");
            };

            sites.push(Connection::new(name, url));
        }

        sites
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    pub fn prev_tab(&mut self) {
        self.selected_tab = self.selected_tab.prev();
    }

    pub fn log_conn(&self) -> (usize, Connection) {
        (
            self.log_conn_idx,
            self.connections
                .lock()
                .unwrap()
                .get(self.log_conn_idx)
                .unwrap()
                .clone(),
        )
    }

    pub fn next_log_conn(&mut self) {
        if self.log_conn_idx != self.connections.lock().unwrap().len() - 1 {
            self.log_conn_idx += 1;
        }
    }

    pub const fn prev_log_conn(&mut self) {
        self.log_conn_idx = self.log_conn_idx.saturating_sub(1);
    }

    pub const fn close(&mut self) {
        self.is_closing = true;
    }

    pub const fn is_closing(&self) -> bool {
        self.is_closing
    }
}
