use async_trait::async_trait;
use tokio_postgres::{Client, Error, Row};
use uuid::Uuid;

mod mutation;
mod query;

use crate::saint_gateway::query::{SaintSortCriteria, SaintSortField, SortDirection};
use domain::boundaries::{
    DbError, SaintCollectionDbResponse, SaintDbRequest, SaintDbResponse,
    SaintSortCriteriaDbRequest, SaintSortDbRequest, SaintSortFieldDbRequest,
    SortDirectionDbRequest,
};
use std::any::Any;

pub struct SaintRepository {
    pub client: Client,
}

#[async_trait]
impl domain::boundaries::SaintDbGateway for SaintRepository {
    async fn get_saint_by_id(&self, id: Uuid) -> Option<SaintDbResponse> {
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
        sort_db_request: Option<SaintSortDbRequest>,
        offset: Option<i64>,
        count: Option<i64>,
    ) -> SaintCollectionDbResponse {
        let display_name = display_name
            // .map(|value| format!("{}", value))
            .unwrap_or("".to_string());
        let offset = offset.unwrap_or(0);
        let count = count.unwrap_or(20);

        let sort_criteria: [Option<SaintSortCriteria>; 5];

        if let Some(sort_db_request) = sort_db_request {
            sort_criteria = to_saint_sort_criteria(sort_db_request)
        } else {
            sort_criteria = [
                Option::from(SaintSortCriteria {
                    field: SaintSortField::DisplayName,
                    direction: SortDirection::ASC,
                }),
                None,
                None,
                None,
                None,
            ];
        }

        let result = query::find_by(
            &(*self).client,
            display_name.clone(),
            is_male,
            sort_criteria,
            count,
            offset,
        )
        .await;
        println!("{:?}", result);
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
        let total_from_offset = query::count_without_limit(
            &(*self).client,
            display_name.clone(),
            is_male.clone(),
            offset,
        )
        .await
        .unwrap();
        if total_from_offset > count {
            has_more = Some(true);
        } else {
            has_more = Some(false);
        }
        let total = query::count_total(&(*self).client, display_name, is_male)
            .await
            .unwrap();
        SaintCollectionDbResponse {
            collection,
            has_more,
            total,
        }
    }
}

fn to_saint_sort_criteria(sort_db_request: SaintSortDbRequest) -> [Option<SaintSortCriteria>; 5] {
    let mut sort_criteria: [Option<SaintSortCriteria>; 5] = [None, None, None, None, None];
    sort_db_request
        .sort_criteria
        .iter()
        .enumerate()
        .for_each(|(index, criterion)| {
            let field = &criterion.field;
            let direction = &criterion.direction;
            sort_criteria[index] = Option::from(SaintSortCriteria {
                field: match field {
                    SaintSortFieldDbRequest::DisplayName => SaintSortField::DisplayName,
                    SaintSortFieldDbRequest::EnglishName => SaintSortField::EnglishName,
                    SaintSortFieldDbRequest::VietnameseName => SaintSortField::VietnameseName,
                    SaintSortFieldDbRequest::FeastDay => SaintSortField::FeastDay,
                    SaintSortFieldDbRequest::FeastMonth => SaintSortField::FeastMonth,
                },
                direction: match direction {
                    SortDirectionDbRequest::ASC => SortDirection::ASC,
                    SortDirectionDbRequest::DESC => SortDirection::DESC,
                },
            });
        });
    sort_criteria
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
