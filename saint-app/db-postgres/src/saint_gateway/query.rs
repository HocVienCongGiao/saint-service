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
    offset: Option<u16>,
    count: Option<u16>,
    is_male: Option<bool>,
    display_name: Option<String>,
) -> Result<Vec<Row>, Error> {
    let display_name = display_name
        .map(|value| format!("%{}%", value))
        .unwrap_or("%".to_string());
    let count = count
        .map(|value| value.to_string())
        .unwrap_or("ALL".to_string());
    let offset = offset.unwrap_or(0);
    let statement: String;
    if is_male.is_some() {
        statement = format!(
            "SELECT * FROM saint__saint_view \
        WHERE display_name LIKE '{}' AND is_male is {} \
        LIMIT {} OFFSET {}",
            display_name,
            is_male.unwrap(),
            count,
            offset
        );
    } else {
        statement = format!(
            "SELECT * FROM saint__saint_view \
        WHERE display_name LIKE '{}' \
        LIMIT {} OFFSET {}",
            display_name, count, offset
        );
    }
    println!("statement = {}", statement);
    let stmt = (*client).prepare(&statement).await.unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let name_param: &[&(dyn ToSql + Sync)] = &[];
    client.query(&stmt, name_param).await
}
