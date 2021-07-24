use tokio_postgres::types::ToSql;
use tokio_postgres::{Client, Error, Row, Transaction};
use uuid::Uuid;

pub(crate) async fn save_id(transaction: &Transaction<'_>, id: Uuid) -> Result<u64, Error> {
    let stmt = (*transaction)
        .prepare("INSERT into public.saint__saint (id) VALUES ($1)")
        .await
        .unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let params: &[&(dyn ToSql + Sync)] = &[&id];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn save_name(
    transaction: &Transaction<'_>,
    id: Uuid,
    field_name: String,
    value: String,
) -> Result<u64, Error> {
    let statement = format!(
        "INSERT into public.saint__saint_{} (id, {}) VALUES ($1, $2)",
        field_name, field_name
    );
    let stmt = (*transaction).prepare(&statement).await.unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let params: &[&(dyn ToSql + Sync)] = &[&id, &value];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn save_gender(
    transaction: &Transaction<'_>,
    id: Uuid,
    is_male: bool,
) -> Result<u64, Error> {
    let stmt = (*transaction)
        .prepare("INSERT into public.saint__saint_gender (id, is_male) VALUES ($1, $2)")
        .await
        .unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let params: &[&(dyn ToSql + Sync)] = &[&id, &is_male];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn save_feast_day(
    transaction: &Transaction<'_>,
    id: Uuid,
    feast_day: i16,
    feast_month: i16,
) -> Result<u64, Error> {
    let stmt = (*transaction)
        .prepare("INSERT into public.saint__saint_feast_day (id, feast_day, feast_month) VALUES ($1, $2, $3)")
        .await
        .unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let params: &[&(dyn ToSql + Sync)] = &[&id, &feast_day, &feast_month];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn update_name(
    client: &Client,
    id: Uuid,
    field_name: String,
    value: String,
) -> Result<u64, Error> {
    let statement = format!(
        "INSERT into public.saint__saint_{} (id, {}) VALUES ($1, $2) \
         ON CONFLICT (id) DO UPDATE SET {} = $2",
        field_name, field_name, field_name
    );
    let stmt = (*client).prepare(&statement).await.unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let params: &[&(dyn ToSql + Sync)] = &[&id, &value];
    client.execute(&stmt, params).await
}

pub(crate) async fn update_gender(client: &Client, id: Uuid, is_male: bool) -> Result<u64, Error> {
    let stmt = (*client)
        .prepare("UPDATE public.saint__saint_gender SET is_male = $2 WHERE id = $1")
        .await
        .unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let params: &[&(dyn ToSql + Sync)] = &[&id, &is_male];
    client.execute(&stmt, params).await
}

pub(crate) async fn update_feast_day(
    client: &Client,
    id: Uuid,
    feast_day: i16,
    feast_month: i16,
) -> Result<u64, Error> {
    let stmt = (*client)
        .prepare("UPDATE public.saint__saint_feast_day SET feast_day = $2, feast_month = $3 WHERE id = $1")
        .await
        .unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let params: &[&(dyn ToSql + Sync)] = &[&id, &feast_day, &feast_month];
    client.execute(&stmt, params).await
}

pub(crate) async fn delete_id(client: &Client, id: Uuid) -> Result<u64, Error> {
    let statement = format!("DELETE FROM public.saint__saint WHERE id = $1");
    let stmt = (*client).prepare(&statement).await.unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let params: &[&(dyn ToSql + Sync)] = &[&id];
    client.execute(&stmt, params).await
}

pub(crate) async fn delete_name(
    client: &Client,
    id: Uuid,
    field_name: String,
) -> Result<u64, Error> {
    let statement = format!(
        "DELETE FROM public.saint__saint_{} WHERE id = $1",
        field_name
    );
    let stmt = (*client).prepare(&statement).await.unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let params: &[&(dyn ToSql + Sync)] = &[&id];
    client.execute(&stmt, params).await
}

pub(crate) async fn delete_gender(client: &Client, id: Uuid) -> Result<u64, Error> {
    let stmt = (*client)
        .prepare("DELETE FROM public.saint__saint_gender WHERE id = $1")
        .await
        .unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let params: &[&(dyn ToSql + Sync)] = &[&id];
    client.execute(&stmt, params).await
}

pub(crate) async fn delete_feast_day(client: &Client, id: Uuid) -> Result<u64, Error> {
    let stmt = (*client)
        .prepare("DELETE FROM public.saint__saint_feast_day WHERE id = $1")
        .await
        .unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let params: &[&(dyn ToSql + Sync)] = &[&id];
    client.execute(&stmt, params).await
}
