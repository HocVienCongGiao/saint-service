use crate::boundaries;
use crate::boundaries::{
    SaintDbGateway, SaintDbRequest, SaintMutationRequest, SaintMutationResponse,
};
use async_trait::async_trait;
use futures::executor::block_on;
use uuid::Uuid;

pub struct SaintMutationInteractor {
    db_gateway: Box<dyn SaintDbGateway>,
}

impl boundaries::SaintMutationInputBoundary for SaintMutationInteractor {
    fn create_saint(&self, request: SaintMutationRequest) -> Option<SaintMutationResponse> {
        println!("saint mutation input boundary {}", request.id.unwrap());
        let mut id: Uuid;
        loop {
            id = Uuid::new_v4();
            if block_on((*self).db_gateway.exists_by_id(id.clone())) {
                println!("this id already exists, continue generate");
            } else {
                break;
            }
        }
        let saint = crate::entity::saint::Saint {
            id: Some(id),
            display_name: request.display_name,
            english_name: request.english_name,
            french_name: request.french_name,
            latin_name: request.latin_name,
            vietnamese_name: request.vietnamese_name,
            gender: request.gender,
            feast_day: request.feast_day,
        };
        if saint.is_valid() {
            println!("This saint is valid");
            let result = block_on((*self).db_gateway.insert(saint.to_saint_db_request()));
            if !result {
                println!("post error");
                return None;
            }
            Some(SaintMutationResponse {})
        } else {
            println!("This saint is not valid");
            None
        }
    }
}

impl SaintMutationInteractor {
    pub fn new(db_gateway: Box<dyn SaintDbGateway>) -> Self {
        SaintMutationInteractor { db_gateway }
    }
}

impl crate::entity::saint::Saint {
    fn to_saint_db_request(&self) -> SaintDbRequest {
        let feast_day: Vec<&str> = self.feast_day.as_ref().unwrap().split('-').collect();
        SaintDbRequest {
            id: self.id.clone(),
            english_name: self.english_name.clone(),
            french_name: self.french_name.clone(),
            latin_name: self.latin_name.clone(),
            vietnamese_name: self.vietnamese_name.clone(),
            display_name: self.display_name.clone(),
            is_male: if self.gender.as_ref().unwrap().eq("male") {
                Some(true)
            } else {
                Some(false)
            },
            feast_day: Some(feast_day[0].parse().unwrap()),
            feast_month: Some(feast_day[1].parse().unwrap()),
        }
    }
}
