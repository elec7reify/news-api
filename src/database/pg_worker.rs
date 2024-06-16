use tokio_postgres::{Client, Error, NoTls};
use crate::database::config::Config;

pub async fn connection() -> Result<Client, Error> {
    let config = Config::new("config.toml").database();
    let connection_string = format!(
        "host={} port={} dbname={} user={} password={}",
        config.host(),
        config.port(),
        config.database(),
        config.username(),
        config.password()
    );
    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });
    Ok(client)
}
