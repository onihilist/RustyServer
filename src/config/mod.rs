use std::fs;
use std::io::Error as IoError;
use futures::TryFutureExt;
use serde::{Deserialize, Serialize};
use crate::utils;
use std::env;
use crate::utils::Logs::UtilsData;

#[derive(Serialize, Deserialize, Debug)]
struct configToml {
    database: Option<configTomlDatabase>
}

#[derive(Serialize, Deserialize, Debug)]
struct configTomlDatabase {
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String
}

#[derive(Debug)]
pub struct configData {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String
}

impl configData {
    pub fn new() -> Self {

        let mut fp: String = "".to_owned();
        let mut content: String = "".to_owned();
        let config_paths: [&str; 2] = [
            "./src/config/config.toml",
            "./src/config/Config.toml"
        ];

        for filepath in config_paths {
            let result: Result<String, IoError> = fs::read_to_string(filepath);
            if result.is_ok() {
                fp = filepath.to_string();
                content = result.unwrap();
                break;
            }
        }

        let configToml: configToml = toml::from_str(&content).unwrap_or_else(|_| {
            let logs: UtilsData = utils::Logs::initLog(None, format!("Config file was found : {}", fp), None);
            utils::Logs::info(logs);
            configToml {
                database: None
            }
        });

        let (host, port, username, password, database):
            (String, u16, String, String, String) = match configToml.database {
            Some(database) => {
                (
                    database.host,
                    database.port,
                    database.username,
                    database.password,
                    database.database
                )
            }
            None => (
                "unknown".to_owned(),
                0.to_owned(),
                "unknown".to_owned(),
                "unknown".to_owned(),
                "unknown".to_owned(),
            )
        };

        configData {
            host,
            port,
            username,
            password,
            database,
        }
    }
}