use crate::openapi::saint::{SaintQueryResponse, ToOpenApi};
use db_postgres::saint_gateway::SaintRepository;
use domain::boundaries::{SaintDbGateway, SaintQueryInputBoundary, SaintQueryRequest};
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
