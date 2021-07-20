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
    let statement = format!(
        "SELECT * FROM saint__saint_view \
        WHERE display_name LIKE {} AND is_male LIKE {} \
        LIMIT {} OFFSET {}",
        display_name.unwrap_or("\'%\'".to_string()),
        is_male
            .map(|value| value.to_string())
            .unwrap_or("\'%\'".to_string()),
        count
            .map(|value| value.to_string())
            .unwrap_or("ALL".to_string()),
        offset.unwrap_or(0)
    );
    let stmt = (*client).prepare(&statement).await.unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let name_param: &[&(dyn ToSql + Sync)] = &[];
    client.query(&stmt, name_param).await
}
