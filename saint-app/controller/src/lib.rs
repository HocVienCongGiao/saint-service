use crate::openapi::saint::{SaintQueryResponse, ToOpenApi};
use db_postgres::saint_gateway::SaintRepository;
use domain::boundaries::{SaintDbGateway, SaintQueryInputBoundary, SaintQueryRequest};
use uuid::Uuid;

pub mod openapi;

pub async fn get_saint() -> openapi::saint::Saint {
    let client = db_postgres::connect().await;

    let saint_repository = SaintRepository { client };

    let response = domain::interactors::saint_query::SaintQueryInteractor::new(saint_repository)
        .get_saint(SaintQueryRequest {
            id: Uuid::parse_str("40e6215db5c64896987cf30f3678f608").unwrap(),
        })
        .await;
    response.to_openapi()
}


pub async fn find_saints_by_id() -> openapi::saint::Saint {
    get_saint().await
}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        crate::find_saints_by_id();
        assert_eq!(2 + 2, 4);
    }
}
