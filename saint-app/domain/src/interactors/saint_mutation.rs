use crate::boundaries;
use crate::boundaries::{SaintDbGateway, SaintMutationRequest, SaintMutationResponse};
use async_trait::async_trait;
use futures::executor::block_on;

pub struct SaintMutationInteractor {
    db_gateway: Box<dyn SaintDbGateway>,
}

impl boundaries::SaintMutationInputBoundary for SaintMutationInteractor {
    fn create_saint(&self, request: SaintMutationRequest) -> SaintMutationResponse {
        println!("saint mutation input boundary {}", request.id);
        SaintMutationResponse {}
    }
}

impl SaintMutationInteractor {
    pub fn new(db_gateway: Box<dyn SaintDbGateway>) -> Self {
        SaintMutationInteractor { db_gateway }
    }
}
