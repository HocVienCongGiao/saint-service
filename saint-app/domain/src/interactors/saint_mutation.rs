use crate::boundaries;
use crate::boundaries::{
    SaintDbGateway, SaintDbRequest, SaintMutationRequest, SaintMutationResponse,
};
use async_trait::async_trait;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

pub struct SaintMutationInteractor<A: SaintDbGateway> {
    db_gateway: A,
}

#[async_trait]
impl<A> boundaries::SaintMutationInputBoundary for SaintMutationInteractor<A>
where
    A: SaintDbGateway + Sync + Send,
{
    async fn create_saint(&self, request: SaintMutationRequest) -> Option<SaintMutationResponse> {
        println!("saint mutation input boundary {}", request.id.unwrap());
        let mut id: Uuid = Uuid::new_v4();
        let mut id_is_valid: bool = false;
        for _ in 0..5 {
            if (*self).db_gateway.exists_by_id(id.clone()).await {
                println!("This id already exists, continue generate");
                id = Uuid::new_v4();
                sleep(Duration::from_millis(500)).await;
            } else {
                id_is_valid = true;
                break;
            }
        }
        if !id_is_valid {
            println!("Can't generate id for this saint");
            return None;
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
            let result = (*self).db_gateway.insert(saint.to_saint_db_request()).await;
            if !result {
                println!("post error");
                return None;
            }
            Some(saint.to_saint_mutation_response())
        } else {
            println!("This saint is not valid");
            None
        }
    }
}

impl<A> SaintMutationInteractor<A>
where
    A: SaintDbGateway + Sync + Send,
{
    pub fn new(db_gateway: A) -> Self {
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

    fn to_saint_mutation_response(&self) -> SaintMutationResponse {
        SaintMutationResponse {
            id: self.id.clone(),
            display_name: self.display_name.clone().unwrap(),
            english_name: self.english_name.clone(),
            french_name: self.french_name.clone(),
            latin_name: self.latin_name.clone(),
            vietnamese_name: self.vietnamese_name.clone().unwrap(),
            gender: self.gender.clone().unwrap(),
            feast_day: self.feast_day.clone().unwrap(),
        }
    }
}
