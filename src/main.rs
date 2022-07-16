use itertools::Itertools;
use regex::Regex;
use serde::Deserialize;
use std::fmt;
use std::fs;

// logging
use env_logger::Env;
use log::{debug, error, info};

// application config
use config::{Config, ConfigError};

const CONFIG_PATH: &str = "Default.toml";

#[derive(Debug, Deserialize, Clone)]
struct AppConf {
    dag: DAGConf,
    log: LogConf,
}

#[derive(Debug, Deserialize, Clone)]
struct DAGConf {
    name: String,
    folder: String,
}

#[derive(Debug, Deserialize, Clone)]
struct LogConf {
    level: String,
}

impl AppConf {
    fn new() -> Result<Self, ConfigError> {
        let mut conf = Config::builder()
            .add_source(config::File::with_name(CONFIG_PATH))
            .build()
            .unwrap();

        let app_conf: AppConf = conf.try_deserialize().unwrap();
        Ok(app_conf)
    }
}

fn error_parsing(_file_name: &str) -> &str {
    return "Unable to parse '{_file_name}'";
}

struct Table<'a> {
    name: &'a str,
}

impl<'a> fmt::Display for Table<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn parse_file(file_name: &str) -> String {
    println!("parsing file: '{file_name}'");
    return fs::read_to_string(file_name).expect(error_parsing(file_name));
}

fn parse_tables(file: &str) -> Vec<String> {
    let re: Regex = Regex::new(r"from\s*(?P<table>[a-zA-Z_]*)").unwrap();
    let file_lower = file.to_lowercase();

    let tables = re.captures_iter(file_lower.as_str()).filter_map(|cap| {
        let group = cap.get(1);
        match group {
            Some(name) => Some(name.as_str()),
            // Some(name) => Some(SelectTable { // NOTE this works too!
            //     name: name.as_str(),
            // }),
            _ => None,
        }
    });
    // tables.map(|m| m.to_string()).collect()
    tables
        .map(|m| m.to_string())
        .into_iter()
        .unique()
        .collect::<Vec<String>>()
}

fn main() {
    let app_conf = AppConf::new().unwrap();
    env_logger::Builder::from_env(Env::default().default_filter_or(app_conf.log.level)).init();

    let files = fs::read_dir(format!("{}/{}/", app_conf.dag.folder, app_conf.dag.name)).unwrap();
    for file in files {
        let file_name = file.unwrap().path().display().to_string(); //.display();
        let parsed = parse_file(file_name.as_str());
        let tables = parse_tables(&parsed);
        debug!("{:#?}", tables);
    }
}
