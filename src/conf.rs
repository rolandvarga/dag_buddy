use serde::Deserialize;

use config::{Config, ConfigError};

const CONFIG_PATH: &str = "Default.toml";

#[derive(Debug, Deserialize, Clone)]
pub struct AppConf {
    pub dag: DAGConf,
    pub log: LogConf,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DAGConf {
    pub name: String,
    pub folder: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LogConf {
    pub level: String,
}

impl AppConf {
    pub fn new() -> Result<Self, ConfigError> {
        let mut conf = Config::builder()
            .add_source(config::File::with_name(CONFIG_PATH))
            .build()
            .unwrap();

        let app_conf: AppConf = conf.try_deserialize().unwrap();
        Ok(app_conf)
    }
}
