use heck::SnakeCase;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Client, Error, Row};
use uuid::Uuid;

pub(crate) struct SaintSortCriteria {
    pub(crate) field: SaintSortField,
    pub(crate) direction: SortDirection,
}

#[derive(strum_macros::Display)]
pub(crate) enum SaintSortField {
    DisplayName,
    VietnameseName,
    EnglishName,
    FeastDay,
    FeastMonth,
}

#[derive(strum_macros::Display)]
pub(crate) enum SortDirection {
    ASC,
    DESC,
}

impl SaintSortCriteria {
    fn to_query_string(&self) -> String {
        let field_str = &*self.field.to_string();
        let field_str_sc = field_str.to_snake_case();
        format!(
            "{} {}",
            field_str_sc.to_lowercase(),
            self.direction.to_string()
        )
    }
}

pub(crate) async fn find_one_by_id(client: &Client, id: Uuid) -> Result<Row, Error> {
    let stmt = (*client)
        .prepare("SELECT * FROM saint__saint_view WHERE id = $1")
        .await
        .unwrap();

    // let stmt = block_on(stmt_future).unwrap();
    let name_param: &[&(dyn ToSql + Sync)] = &[&id];
    client.query_one(&stmt, name_param).await
}

pub(crate) async fn find_by(
    client: &Client,
    display_name: String,
    is_male: Option<bool>,
    order_by_criteria: [Option<SaintSortCriteria>; 5],
    count: i64,
    offset: i64,
) -> Result<Vec<Row>, Error> {
    let order_by_string: String;
    let mut order_by_strings: Vec<String> = Vec::new();
    for (i, e) in order_by_criteria.iter().enumerate() {
        if let Some(element) = e {
            order_by_strings.push(element.to_query_string());
        }
    }
    order_by_string = order_by_strings.join(", ");

    let statement = format!(
        "SELECT * FROM saint__saint_view \
        WHERE unaccent(display_name) LIKE ('%' || unaccent($1) || '%') AND ($2::BOOL is null or is_male = $2::BOOL) \
        ORDER BY {} \
        LIMIT $3 OFFSET $4", 
        order_by_string
    );

    println!("statement = {}", statement);
    let stmt = (*client).prepare(&statement).await.unwrap();
    let name_param: &[&(dyn ToSql + Sync)] = &[&display_name, &is_male, &count, &offset];
    client.query(&stmt, name_param).await
}

pub async fn count_without_limit(
    client: &Client,
    display_name: String,
    is_male: Option<bool>,
    offset: i64,
) -> Result<i64, Error> {
    let statement = format!(
        "SELECT COUNT(*) FROM
        (SELECT * FROM saint__saint_view \
        WHERE unaccent(display_name) LIKE ('%' || unaccent($1) || '%') AND ($2::BOOL is null or is_male = $2::BOOL) \
        ORDER BY id \
        LIMIT ALL OFFSET $3) AS saints",
    );

    println!("statement = {}", statement);
    let stmt = (*client).prepare(&statement).await.unwrap();
    let name_param: &[&(dyn ToSql + Sync)] = &[&display_name, &is_male, &offset];
    Ok(client.query_one(&stmt, name_param).await?.get("count"))
}

pub async fn count_total(
    client: &Client,
    display_name: String,
    is_male: Option<bool>,
) -> Result<i64, Error> {
    let statement = format!(
        "SELECT COUNT(*) FROM saint__saint_view \
        WHERE unaccent(display_name) LIKE ('%' || unaccent($1) || '%') AND ($2::BOOL is null or is_male = $2::BOOL)",
    );

    println!("statement = {}", statement);
    let stmt = (*client).prepare(&statement).await.unwrap();
    let name_param: &[&(dyn ToSql + Sync)] = &[&display_name, &is_male];
    Ok(client.query_one(&stmt, name_param).await?.get("count"))
}
