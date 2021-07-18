use crate::openapi::saint::{SaintQueryResponse, ToOpenApi};
use db_postgres::saint_gateway::SaintRepository;
use domain::boundaries::{
    SaintDbGateway, SaintMutationError, SaintMutationInputBoundary, SaintMutationRequest,
    SaintQueryInputBoundary, SaintQueryRequest,
};
pub use hvcg_biography_openapi_saint::models::Saint;
use uuid::Uuid;

pub mod openapi;

pub async fn get_saint(id: Uuid) -> Option<openapi::saint::Saint> {
    let client = db_postgres::connect().await;

    let saint_repository = SaintRepository { client };

    let response = domain::interactors::saint_query::SaintQueryInteractor::new(saint_repository)
        .get_saint(SaintQueryRequest { id: id.clone() })
        .await;
    if response.is_none() {
        return None;
    }
    Some(response.unwrap().to_openapi())
}

pub async fn create_saint(saint: &Saint) -> Result<openapi::saint::Saint, SaintMutationError> {
    let client = db_postgres::connect().await;

    let saint_repository = SaintRepository { client };

    let response =
        domain::interactors::saint_mutation::SaintMutationInteractor::new(saint_repository)
            .create_saint(SaintMutationRequest {
                id: None,
                display_name: Some(saint.display_name.clone()),
                english_name: saint.english_name.clone(),
                french_name: saint.french_name.clone(),
                latin_name: saint.latin_name.clone(),
                vietnamese_name: Some(saint.vietnamese_name.clone()),
                gender: Some(saint.gender.clone()),
                feast_day: Some(saint.feast_day.clone()),
            })
            .await;
    response.map(|res| res.to_openapi())
}

pub async fn update_saint(
    saint: &Saint,
    id: Uuid,
) -> Result<openapi::saint::Saint, SaintMutationError> {
    let client = db_postgres::connect().await;

    let saint_repository = SaintRepository { client };

    let response =
        domain::interactors::saint_mutation::SaintMutationInteractor::new(saint_repository)
            .update_saint(SaintMutationRequest {
                id: Some(id),
                display_name: Some(saint.display_name.clone()),
                english_name: saint.english_name.clone(),
                french_name: saint.french_name.clone(),
                latin_name: saint.latin_name.clone(),
                vietnamese_name: Some(saint.vietnamese_name.clone()),
                gender: Some(saint.gender.clone()),
                feast_day: Some(saint.feast_day.clone()),
            })
            .await;
    response.map(|res| res.to_openapi())
}

pub async fn delete_saint(id: Uuid) -> Result<(), SaintMutationError> {
    let client = db_postgres::connect().await;

    let saint_repository = SaintRepository { client };

    let response =
        domain::interactors::saint_mutation::SaintMutationInteractor::new(saint_repository)
            .delete_saint(SaintMutationRequest {
                id: Some(id),
                display_name: None,
                english_name: None,
                french_name: None,
                latin_name: None,
                vietnamese_name: None,
                gender: None,
                feast_day: None,
            })
            .await;
    response
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
