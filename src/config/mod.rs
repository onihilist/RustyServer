
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

        let config_paths: [&str; 2] = [
            "./config.toml",
            "./Config.toml"
        ];
        configData {
            host: "".to_owned(),
            port: 0.to_owned(),
            username: "".to_owned(),
            password: "".to_owned(),
            database: "".to_owned(),
        }
    }
}