use tokio_postgres::{NoTls, Row};
use std::env;
use crate::utils;

pub async fn connect_to_db() -> Result<tokio_postgres::Client, tokio_postgres::Error> {

    let config: configData = configData::new();

    let host = env::var("HOST").unwrap();
    let port: u16 = env::var("PORT").expect("PORT must be set").parse().expect("PORT must be a number");
    let username = env::var("USERNAME").expect("USERNAME must be set");
    let password = env::var("PASSWORD").expect("PASSWORD must be set");
    let database = env::var("DATABASE").expect("DATABASE must be set");

    let (client, connection) =
        tokio_postgres::connect(format!("host={} port={} user={} password={} dbname={}", host, port, username, password, database).as_str(), NoTls)
            .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            let logs = utils::Logs::initLog(None, format!("Impossible to connect to the database : {}", e), None);
            utils::Logs::error(logs);
        }
    });

    Ok(client)
}

async fn execute_query(client: &tokio_postgres::Client, query: &str, params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)>) -> Result<Vec<Row>, tokio_postgres::Error> {
    client.query(query, &params).await
}
/*
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
pub async fn dumpAll(client: &tokio_postgres::Client, params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)>) -> Vec<Row> {
    execute_query(client,
                  "SELECT * FROM users;",
                  vec![]).await.unwrap()
}