use domain::test_func;
use tokio_postgres::{Client, Error, NoTls};

pub mod config;
pub mod saint_gateway;
pub async fn connect() -> Client {
    let config = config::Config::new();
    let result = tokio_postgres::connect(
        format!(
            "user={} password={} host={} port={} dbname={}",
            config.db_user, config.db_password, config.db_host, config.db_port, config.db_name
        )
        .as_str(),
        //         tokio_postgres::connect("postgresql://postgres:password@localhost/test", NoTls).await?;
        NoTls,
    )
    .await;

    let (client, connection) = result.unwrap();
    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    client
    // p%40ssword
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn initialise() {
        INIT.call_once(|| {
            let my_path = PathBuf::new().join(".env.test");
            dotenv::from_path(my_path.as_path()).ok();
            println!("testing env {}", std::env::var("HELLO").unwrap());
        });
    }

    #[tokio::test]
    async fn it_works() {
        initialise();
        assert_eq!(2 + 2, 4);
        println!("finished saint");
    }
}
