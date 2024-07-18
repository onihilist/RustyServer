use std::fs;
use std::io::Error as IoError;
use futures::TryFutureExt;
use serde::{Deserialize, Serialize};
use crate::utils;
use std::env;
use crate::utils::Logs::UtilsData;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct configToml {
    pub database: Option<configTomlDatabase>,
    pub server: Option<configTomlServer>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct configTomlDatabase {
    pub typedb: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String
}

#[derive(Debug)]
pub struct configData {
    pub typedb: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct configTomlServer {
    pub maxUser: u16,
    pub timeout: u32
}

#[derive(Debug)]
pub struct configServer {
    pub maxUser: u16,
    pub timeout: u32
}


impl configToml {
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
                database: None,
                server: None,
            }
        });

        let (typedb, host, port, username, password, database):
            (String, String, u16, String, String, String) = match configToml.database {
            Some(database) => {
                (
                    database.typedb,
                    database.host,
                    database.port,
                    database.username,
                    database.password,
                    database.database
                )
            }
            None => (
                "unknown".to_owned(),
                "unknown".to_owned(),
                0.to_owned(),
                "unknown".to_owned(),
                "unknown".to_owned(),
                "unknown".to_owned(),
            )
        };

        let (maxUser, timeout):
            (u16, u32) = match configToml.server {
            Some(server) => {
                (
                    server.maxUser,
                    server.timeout
                )
            }
            None => (
                10.to_owned(),
                10000.to_owned(),
            )
        };

        configToml {
            database: Some(configTomlDatabase {
                        typedb,
                        host,
                        port,
                        username,
                        password,
                        database,
                        }),
            server: Some(configTomlServer{
                maxUser,
                timeout
            }),
        }
    }
}