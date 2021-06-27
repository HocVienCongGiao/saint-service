use tokio_postgres::types::ToSql;
use tokio_postgres::{Client, Error, Row};
use uuid::Uuid;

pub async fn find_one_by_id(client: &Client, id: Uuid) -> Result<Row, Error> {
    let stmt = (*client)
        .prepare("SELECT * FROM saint__saint_view WHERE id = $1")
        .await
        .unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let name_param: &[&(dyn ToSql + Sync)] = &[&id];
    client.query_one(&stmt, name_param).await
}

pub async fn save(client: &Client, name: String, country: String) -> Result<u64, Error> {
    let stmt = (*client)
        .prepare("INSERT into example__author_initial(name, country) VALUES ($1, $2)")
        .await
        .unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let params: &[&(dyn ToSql + Sync)] = &[&name, &country];
    client.execute(&stmt, params).await
}
