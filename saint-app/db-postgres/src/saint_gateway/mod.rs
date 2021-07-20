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

    async fn insert(&self, db_request: SaintDbRequest) -> Result<(), DbError> {
        let mut result: Result<u64, Error>;

        let id = db_request.id.unwrap();
        result = mutation::save_id(&(*self).client, id.clone()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        let display_name = db_request.display_name.unwrap();
        result = mutation::save_name(
            &(*self).client,
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
                &(*self).client,
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
                &(*self).client,
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
                &(*self).client,
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
            &(*self).client,
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
        result = mutation::save_gender(&(*self).client, id.clone(), is_male.clone()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        let feast_day = db_request.feast_day.unwrap();
        let feast_month = db_request.feast_month.unwrap();
        result = mutation::save_feast_day(
            &(*self).client,
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
        Ok(())
    }

    async fn update(&self, db_request: SaintDbRequest) -> Result<(), DbError> {
        let mut result: Result<u64, Error>;

        let id = db_request.id.unwrap();

        let display_name = db_request.display_name.unwrap();
        result = mutation::update_name(
            &(*self).client,
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
                &(*self).client,
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
                &(*self).client,
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
                &(*self).client,
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
            &(*self).client,
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
        result = mutation::update_gender(&(*self).client, id.clone(), is_male.clone()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        let feast_day = db_request.feast_day.unwrap();
        let feast_month = db_request.feast_month.unwrap();
        result = mutation::update_feast_day(
            &(*self).client,
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
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DbError> {
        let mut result: Result<u64, Error>;

        result =
            mutation::delete_name(&(*self).client, id.clone(), "display_name".to_string()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        result =
            mutation::delete_name(&(*self).client, id.clone(), "english_name".to_string()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        result =
            mutation::delete_name(&(*self).client, id.clone(), "french_name".to_string()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        result = mutation::delete_name(&(*self).client, id.clone(), "latin_name".to_string()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        result =
            mutation::delete_name(&(*self).client, id.clone(), "vietnamese_name".to_string()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        result = mutation::delete_gender(&(*self).client, id.clone()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        result = mutation::delete_feast_day(&(*self).client, id.clone()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        result = mutation::delete_id(&(*self).client, id.clone()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        Ok(())
    }

    async fn get_collection_saint(
        &self,
        is_male: Option<bool>,
        display_name: Option<String>,
        offset: Option<u16>,
        count: Option<u16>,
    ) -> SaintCollectionDbResponse {
        let result =
            query::get_collection(&(*self).client, offset, count, is_male, display_name).await;
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
        SaintCollectionDbResponse { collection }
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
