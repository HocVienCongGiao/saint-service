use db_postgres::saint_gateway::SaintRepository;
use domain::boundaries::SaintDbGateway;
// use pg_embed::postgres::PgEmbed;
use std::path::PathBuf;
use std::sync::Once;

mod common;

static INIT: Once = Once::new();

fn initialise() {
    INIT.call_once(|| {
        let my_path = PathBuf::new().join(".env.test");
        dotenv::from_path(my_path.as_path()).ok();
        println!("testing env {}", std::env::var("HELLO").unwrap());
    });
}

#[tokio::test]
async fn integration_works() {
    initialise();
    println!("is it working?");
    // let mut pg: PgEmbed = common::embedded::start_postgres().await;
    let client = db_postgres::connect().await;
    let client = db_postgres::main(client).await.unwrap();
    let saint_repository = SaintRepository { client };
    let is_existing = saint_repository
        .exists_by_id("Ngo Dinh Diem".to_string())
        .await;
    println!("is existing is {}", is_existing);
    assert_eq!(2 + 2, 4);
    println!("finished integration test");
    // let _ = pg.stop_db();
}

#[tokio::test]
async fn save_test() {
    initialise();
    println!("is it working?");
    // let mut pg: PgEmbed = common::embedded::start_postgres().await;
    let client = db_postgres::connect().await;
    let client = db_postgres::main(client).await.unwrap();
    let saint_repository = SaintRepository { client };
    let result = saint_repository
        .insert("NhutHuynh".parse().unwrap(), "HK".parse().unwrap())
        .await;
    println!("Is insert successfully {}", result);
    let is_existing = saint_repository.exists_by_id("NhutHuynh".to_string()).await;
    println!("is existing NhutHuynh is {}", is_existing);
    println!("finished integration test");
    // let _ = pg.stop_db();
}

// #[tokio::test]
// async fn db_tests() {
//     println!("start db_tests");
//     initialise();
//     let mut pg = crate::embedded::start_postgres().await;
//     main().await.expect("Failing for no reason");
//     println!("Finished main() in test 2");
//     assert_eq!(2 + 2, 4);
//     println!("finished test2");
//     let _ = pg.stop_db();
// }
