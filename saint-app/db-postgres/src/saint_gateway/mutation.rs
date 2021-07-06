use tokio_postgres::types::ToSql;
use tokio_postgres::{Client, Error, Row};
use uuid::Uuid;

pub async fn save_id(client: &Client, id: Uuid) -> Result<u64, Error> {
    let stmt = (*client)
        .prepare("INSERT into public.saint__saint (id) VALUES ($1)")
        .await
        .unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let params: &[&(dyn ToSql + Sync)] = &[&id];
    client.execute(&stmt, params).await
}

pub async fn save_name(
    client: &Client,
    id: Uuid,
    field_name: String,
    value: String,
) -> Result<u64, Error> {
    let stmt = (*client)
        .prepare("INSERT into public.saint__saint_$1 (id, $1) VALUES ($2, $3)")
        .await
        .unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let params: &[&(dyn ToSql + Sync)] = &[&field_name, &id, &value];
    client.execute(&stmt, params).await
}

pub async fn save_gender(client: &Client, id: Uuid, is_male: bool) -> Result<u64, Error> {
    let stmt = (*client)
        .prepare("INSERT into public.saint__saint_gender (id, is_male) VALUES ($1, $2)")
        .await
        .unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let params: &[&(dyn ToSql + Sync)] = &[&id, &is_male];
    client.execute(&stmt, params).await
}

pub async fn save_feast_day(
    client: &Client,
    id: Uuid,
    feast_day: i16,
    feast_month: i16,
) -> Result<u64, Error> {
    let stmt = (*client)
        .prepare("public.saint__saint_feast_day (id, feast_day, feast_month) VALUES ($1, $2, $3)")
        .await
        .unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let params: &[&(dyn ToSql + Sync)] = &[&id, &feast_day, &feast_month];
    client.execute(&stmt, params).await
}
