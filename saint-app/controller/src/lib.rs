use crate::openapi::saint::{SaintQueryResponse, ToOpenApi};
use db_postgres::saint_gateway::SaintRepository;
use domain::boundaries::{
    SaintDbGateway, SaintMutationInputBoundary, SaintMutationRequest, SaintQueryInputBoundary,
    SaintQueryRequest,
};
use std::collections::HashMap;
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

pub async fn create_saint(
    saint_obj: HashMap<String, Option<String>>,
) -> Option<openapi::saint::Saint> {
    let client = db_postgres::connect().await;

    let saint_repository = SaintRepository { client };

    let response =
        domain::interactors::saint_mutation::SaintMutationInteractor::new(saint_repository)
            .create_saint(SaintMutationRequest {
                id: None,
                display_name: saint_obj.get("display_name").unwrap().clone(),
                english_name: saint_obj.get("english_name").unwrap().clone(),
                french_name: saint_obj.get("french_name").unwrap().clone(),
                latin_name: saint_obj.get("latin_name").unwrap().clone(),
                vietnamese_name: saint_obj.get("vietnamese_name").unwrap().clone(),
                gender: saint_obj.get("gender").unwrap().clone(),
                feast_day: saint_obj.get("feast_day").unwrap().clone(),
            })
            .await;
    if response.is_none() {
        return None;
    }
    Some(response.unwrap().to_openapi())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
