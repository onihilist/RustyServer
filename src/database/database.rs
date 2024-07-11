use std::panic::resume_unwind;
use crate::config::{configToml, configTomlDatabase};
use tokio_postgres::{NoTls, Row};
use crate::server::users::userData;
use crate::utils;

const CONFIG: configToml = configToml::new();

pub async fn connectToDB() -> Result<tokio_postgres::Client, tokio_postgres::Error> {

    let db: configTomlDatabase = CONFIG.database.unwrap();

    let connection_string = format!("host={} port={} user={} password={}", // dbname={}",
                                    db.host,
                                    db.port,
                                    db.username.trim(),
                                    db.password.trim(),
                                    );

    match tokio_postgres::connect(&connection_string, NoTls).await {
        Ok((client, connection)) => {
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    let logs = utils::Logs::initLog(None, format!("Impossible to connect to the database : {}", e), None);
                    utils::Logs::error(logs);
                }
            });
            let logs = utils::Logs::initLog(None, format!("Connected to the database ({}:{})", db.host, db.port), None);
            utils::Logs::success(logs);
            Ok(client)
        }
        Err(e) => {
            let error_message = format!("Impossible to connect to the database : {}", e);
            let logs = utils::Logs::initLog(None, error_message, None);
            utils::Logs::error(logs);
            Err(e)
        }
    }
}

async fn executeQuery(client: tokio_postgres::Client, query: &str, params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)>) -> Result<Vec<Row>, tokio_postgres::Error> {
    client.query(query, &params).await
}

/*
pub async fn dumpAll(client: tokio_postgres::Client, params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)>) -> Vec<userData> {
    let rows = executeQuery(client, "SELECT * FROM users;", vec![]).await.unwrap();
    let mut users = Vec::new();
    for row in rows {
        let userId: i32 = row.get(0);
        let hwid: String = match row.get(1) {
            Some(hwid) => hwid.unwrap().to_string(),
            None => "Unknown HWID".to_string(),
        };
        let username: String = match row.get(2) {
            Some(username) => username.unwrap().to_string(),
            None => "Unknown username".to_string(),
        };
        let pass: String = match row.get(3) {
            Some(pass) => pass.unwrap().to_string(),
            None => "Unknown password".to_string(),
        };
        let user = userData {
            userId,
            hwid,
            username,
            pass,
        };
        users.push(user);
    }
    users
}

pub async fn createAccount(client: &tokio_postgres::Client, request: protocolData, params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)>) -> Vec<Row> {
    execute_query(client,
                  "INSERT INTO users (hwid, username, pass, creationDate) VALUES ($1, $2, $3, $4);",
                  vec![
                      &(1454212i32 as &(dyn tokio_postgres::types::ToSql + Sync)),
                      &(request.sender as &(dyn tokio_postgres::types::ToSql + Sync)),
                      &("password" as &(dyn tokio_postgres::types::ToSql + Sync)),
                      &(215112152i32 as &(dyn tokio_postgres::types::ToSql + Sync))
                  ]).await.unwrap()
}
*/