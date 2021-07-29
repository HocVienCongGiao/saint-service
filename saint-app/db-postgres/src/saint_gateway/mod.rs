use async_trait::async_trait;
use domain::boundaries::DbError;
use tokio_postgres::{Client, Error, Row};
use uuid::Uuid;

mod mutation;
mod query;

use domain::boundaries::{SaintCollectionDbResponse, SaintDbRequest, SaintDbResponse};

pub struct SaintRepository {
    pub client: Client,
}

#[async_trait]
impl domain::boundaries::SaintDbGateway for SaintRepository {
    async fn find_by_id(&self, id: Uuid) -> Option<SaintDbResponse> {
        let result = query::find_one_by_id(&(*self).client, id.clone()).await;
        println!("second block_on for row");
        if result.is_err() {
            return None;
        }
        let row = result.unwrap();
        Some(convert_to_saint_db_response(row))
    }

    async fn exists_by_id(&self, id: Uuid) -> bool {
        let result = query::find_one_by_id(&(*self).client, id.clone()).await;
        println!("second block_on for row");
        if result.is_err() {
            return false;
        }
        let row = result.unwrap();
        let id_found: Option<Uuid> = row.get("id");
        println!("ROW IS {}", id);
        if id_found.is_none() {
            return false;
        }
        id_found.unwrap() == id
    }

    async fn insert(&mut self, db_request: SaintDbRequest) -> Result<(), DbError> {
        let mut result: Result<u64, Error>;

        let transaction = (*self).client.transaction().await.or_else(|error| {
            Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ))
        })?;

        let id = db_request.id.unwrap();
        result = mutation::save_id(&transaction, id.clone()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        let display_name = db_request.display_name.unwrap();
        result = mutation::save_name(
            &transaction,
            id.clone(),
            "display_name".to_string(),
            display_name.clone(),
        )
        .await;
        if result.is_err() {
            return Err(DbError::UniqueConstraintViolationError(
                "display_name".to_string(),
            ));
        }
        if let Some(english_name) = db_request.english_name {
            result = mutation::save_name(
                &transaction,
                id.clone(),
                "english_name".to_string(),
                english_name.clone(),
            )
            .await;
            if result.is_err() {
                return Err(DbError::UniqueConstraintViolationError(
                    "english_name".to_string(),
                ));
            }
        }
        if let Some(french_name) = db_request.french_name {
            result = mutation::save_name(
                &transaction,
                id.clone(),
                "french_name".to_string(),
                french_name.clone(),
            )
            .await;
            if result.is_err() {
                return Err(DbError::UniqueConstraintViolationError(
                    "french_name".to_string(),
                ));
            }
        }
        if let Some(latin_name) = db_request.latin_name {
            result = mutation::save_name(
                &transaction,
                id.clone(),
                "latin_name".to_string(),
                latin_name.clone(),
            )
            .await;
            if result.is_err() {
                return Err(DbError::UniqueConstraintViolationError(
                    "latin_name".to_string(),
                ));
            }
        }
        let vietnamese_name = db_request.vietnamese_name.unwrap();
        result = mutation::save_name(
            &transaction,
            id.clone(),
            "vietnamese_name".to_string(),
            vietnamese_name.clone(),
        )
        .await;
        if result.is_err() {
            return Err(DbError::UniqueConstraintViolationError(
                "vietnamese_name".to_string(),
            ));
        }
        let is_male = db_request.is_male.unwrap();
        result = mutation::save_gender(&transaction, id.clone(), is_male.clone()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        let feast_day = db_request.feast_day.unwrap();
        let feast_month = db_request.feast_month.unwrap();
        result = mutation::save_feast_day(
            &transaction,
            id.clone(),
            feast_day.clone(),
            feast_month.clone(),
        )
        .await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        transaction
            .commit()
            .await
            .map_err(|error| DbError::UnknownError(error.into_source().unwrap().to_string()))
    }

    async fn update(&mut self, db_request: SaintDbRequest) -> Result<(), DbError> {
        let mut result: Result<u64, Error>;

        let transaction = (*self).client.transaction().await.or_else(|error| {
            Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ))
        })?;

        let id = db_request.id.unwrap();

        let display_name = db_request.display_name.unwrap();
        result = mutation::update_name(
            &transaction,
            id.clone(),
            "display_name".to_string(),
            display_name.clone(),
        )
        .await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        if let Some(english_name) = db_request.english_name {
            result = mutation::update_name(
                &transaction,
                id.clone(),
                "english_name".to_string(),
                english_name.clone(),
            )
            .await;
            if let Err(error) = result {
                return Err(DbError::UnknownError(
                    error.into_source().unwrap().to_string(),
                ));
            }
        }
        if let Some(french_name) = db_request.french_name {
            result = mutation::update_name(
                &transaction,
                id.clone(),
                "french_name".to_string(),
                french_name.clone(),
            )
            .await;
            if let Err(error) = result {
                return Err(DbError::UnknownError(
                    error.into_source().unwrap().to_string(),
                ));
            }
        }
        if let Some(latin_name) = db_request.latin_name {
            result = mutation::update_name(
                &transaction,
                id.clone(),
                "latin_name".to_string(),
                latin_name.clone(),
            )
            .await;
            if let Err(error) = result {
                return Err(DbError::UnknownError(
                    error.into_source().unwrap().to_string(),
                ));
            }
        }
        let vietnamese_name = db_request.vietnamese_name.unwrap();
        result = mutation::update_name(
            &transaction,
            id.clone(),
            "vietnamese_name".to_string(),
            vietnamese_name.clone(),
        )
        .await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        let is_male = db_request.is_male.unwrap();
        result = mutation::update_gender(&transaction, id.clone(), is_male.clone()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        let feast_day = db_request.feast_day.unwrap();
        let feast_month = db_request.feast_month.unwrap();
        result = mutation::update_feast_day(
            &transaction,
            id.clone(),
            feast_day.clone(),
            feast_month.clone(),
        )
        .await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        transaction
            .commit()
            .await
            .map_err(|error| DbError::UnknownError(error.into_source().unwrap().to_string()))
    }

    async fn delete(&mut self, id: Uuid) -> Result<(), DbError> {
        let mut result: Result<u64, Error>;

        let transaction = (*self).client.transaction().await.or_else(|error| {
            Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ))
        })?;

        result = mutation::delete_name(&transaction, id.clone(), "display_name".to_string()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        result = mutation::delete_name(&transaction, id.clone(), "english_name".to_string()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        result = mutation::delete_name(&transaction, id.clone(), "french_name".to_string()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        result = mutation::delete_name(&transaction, id.clone(), "latin_name".to_string()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        result =
            mutation::delete_name(&transaction, id.clone(), "vietnamese_name".to_string()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        result = mutation::delete_gender(&transaction, id.clone()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        result = mutation::delete_feast_day(&transaction, id.clone()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        result = mutation::delete_id(&transaction, id.clone()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        transaction
            .commit()
            .await
            .map_err(|error| DbError::UnknownError(error.into_source().unwrap().to_string()))
    }

    async fn get_saint_collection(
        &self,
        is_male: Option<bool>,
        display_name: Option<String>,
        offset: Option<u16>,
        count: Option<u16>,
    ) -> SaintCollectionDbResponse {
        let filter = parse_filter_param(offset, count, is_male, display_name.clone());
        let result = query::get_collection(&(*self).client, filter).await;

        let collection: Vec<SaintDbResponse>;
        if result.is_err() {
            collection = vec![];
        } else {
            collection = result
                .unwrap()
                .into_iter()
                .map(|row| convert_to_saint_db_response(row))
                .collect();
        }

        let has_more: Option<bool>;
        if let Some(count_param) = count {
            let filter = parse_filter_param(offset, None, is_male, display_name.clone());
            let count_result = query::count_without_limit(&(*self).client, filter)
                .await
                .unwrap();
            if (count_result as u16) > count_param {
                has_more = Some(true);
            } else {
                has_more = Some(false);
            }
        } else {
            has_more = None
        };
        SaintCollectionDbResponse {
            collection,
            has_more,
        }
    }
}

fn convert_to_saint_db_response(row: Row) -> SaintDbResponse {
    let id: Option<Uuid>;
    let english_name: Option<String>;
    let french_name: Option<String>;
    let latin_name: Option<String>;

    if let Some(value) = row.get("id") {
        id = Some(value)
    } else {
        id = None
    };
    if let Some(value) = row.get("english_name") {
        english_name = Some(value)
    } else {
        english_name = None
    };
    if let Some(value) = row.get("french_name") {
        french_name = Some(value)
    } else {
        french_name = None
    };
    if let Some(value) = row.get("latin_name") {
        latin_name = Some(value)
    } else {
        latin_name = None
    };
    let vietnamese_name: String = row.get("vietnamese_name");
    let display_name: String = row.get("display_name");
    let is_male: bool = row.get("is_male");
    let feast_day: i16 = row.get("feast_day");
    let feast_month: i16 = row.get("feast_month");

    SaintDbResponse {
        id,
        english_name,
        french_name,
        latin_name,
        vietnamese_name,
        display_name,
        is_male,
        feast_day,
        feast_month,
    }
}

fn parse_filter_param(
    offset: Option<u16>,
    count: Option<u16>,
    is_male: Option<bool>,
    display_name: Option<String>,
) -> String {
    let display_name = display_name
        .map(|value| format!("%{}%", value))
        .unwrap_or("%".to_string());
    let count = count
        .map(|value| value.to_string())
        .unwrap_or("ALL".to_string());
    let offset = offset.unwrap_or(0);

    if is_male.is_some() {
        format!(
            "WHERE display_name LIKE '{}' AND is_male is {} \
        LIMIT {} OFFSET {}",
            display_name,
            is_male.unwrap(),
            count,
            offset
        )
    } else {
        format!(
            "WHERE display_name LIKE '{}' \
        LIMIT {} OFFSET {}",
            display_name, count, offset
        )
    }
}
