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

pub async fn get_collection(
    client: &Client,
    display_name: String,
    is_male: Option<bool>,
    count: i64,
    offset: i64,
) -> Result<Vec<Row>, Error> {
    let statement: String;
    if is_male.is_some() {
        statement = format!(
            "SELECT * FROM saint__saint_view \
            WHERE display_name LIKE $1 AND is_male is $2 \
            ORDER BY id \
            LIMIT $3 OFFSET $4",
        );

        println!("statement = {}", statement);
        let stmt = (*client).prepare(&statement).await.unwrap();
        let name_param: &[&(dyn ToSql + Sync)] =
            &[&display_name, &is_male.unwrap(), &count, &offset];
        client.query(&stmt, name_param).await
    } else {
        statement = format!(
            "SELECT * FROM saint__saint_view \
            WHERE display_name LIKE $1 \
            ORDER BY id \
            LIMIT $2 OFFSET $3",
        );

        println!("statement = {}", statement);
        let stmt = (*client).prepare(&statement).await.unwrap();
        let name_param: &[&(dyn ToSql + Sync)] = &[&display_name, &count, &offset];
        client.query(&stmt, name_param).await
    }
}

pub async fn count_without_limit(
    client: &Client,
    display_name: String,
    is_male: Option<bool>,
    offset: i64,
) -> Result<i64, Error> {
    let statement: String;
    if is_male.is_some() {
        statement = format!(
            "SELECT COUNT(*) FROM
            (SELECT * FROM saint__saint_view \
            WHERE display_name LIKE $1 AND is_male is $2 \
            ORDER BY id \
            LIMIT ALL OFFSET $3) AS saints",
        );

        println!("statement = {}", statement);
        let stmt = (*client).prepare(&statement).await.unwrap();
        let name_param: &[&(dyn ToSql + Sync)] = &[&display_name, &is_male.unwrap(), &offset];
        Ok(client.query_one(&stmt, name_param).await?.get("count"))
    } else {
        statement = format!(
            "SELECT COUNT(*) FROM
            (SELECT * FROM saint__saint_view \
            WHERE display_name LIKE $1 \
            ORDER BY id \
            LIMIT ALL OFFSET $2) AS saints",
        );

        println!("statement = {}", statement);
        let stmt = (*client).prepare(&statement).await.unwrap();
        let name_param: &[&(dyn ToSql + Sync)] = &[&display_name, &offset];
        Ok(client.query_one(&stmt, name_param).await?.get("count"))
    }
}
