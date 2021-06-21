use crate::boundaries;
use crate::boundaries::{SaintDbGateway, SaintMutationRequest, SaintMutationResponse};
use async_trait::async_trait;
use futures::executor::block_on;

pub struct SaintMutationInteractor {
    db_gateway: Box<dyn SaintDbGateway>,
}

impl boundaries::SaintMutationInputBoundary for SaintMutationInteractor {
    fn create_saint(&self, request: SaintMutationRequest) -> SaintMutationResponse {
        println!("saint mutation input boundary {}", request.name);
        if block_on((*self).db_gateway.exists_by_name(request.name.clone())) {
            println!("user with this name already exists");
        } else {
            println!("new user, all is good");
            let test1 = crate::entity::saint::Saint {
                id: 0,
                name: request.name,
            };
            if test1.is_valid() {
                println!("This user is valid");
            }
        }
        SaintMutationResponse {}
    }
}

impl SaintMutationInteractor {
    pub fn new(db_gateway: Box<dyn SaintDbGateway>) -> Self {
        SaintMutationInteractor { db_gateway }
    }
}
