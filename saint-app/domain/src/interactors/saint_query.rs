use crate::boundaries;
use crate::boundaries::{SaintDbGateway, SaintQueryRequest, SaintQueryResponse, SaintDbResponse};
use async_trait::async_trait;

pub struct SaintQueryInteractor<A: SaintDbGateway> {
    db_gateway: A,
}

#[async_trait]
impl<A> boundaries::SaintQueryInputBoundary for SaintQueryInteractor<A>
where
    A: SaintDbGateway + Sync + Send,
{
    async fn get_saint(&self, request: SaintQueryRequest) -> Option<SaintQueryResponse> {
        println!("saint mutation input boundary {}", request.id);

        if let Some(db_response) = ((*self).db_gateway.find_by_id(request.id.clone())).await {
            println!("saint found");
            return Some(db_response.to_saint_query_response());
        } else {
            println!("saint not found");
            return None
        }
    }
}

impl<A> SaintQueryInteractor<A>
where
    A: SaintDbGateway + Sync + Send,
{
    pub fn new(db_gateway: A) -> Self {
        SaintQueryInteractor { db_gateway }
    }
}


impl SaintDbResponse {
    fn to_saint_query_response(&self) -> SaintQueryResponse {
        SaintQueryResponse {
            id: self.id.clone(),
            english_name: self.english_name.clone(),
            french_name: self.french_name.clone(),
            latin_name: self.latin_name.clone(),
            vietnamese_name: self.vietnamese_name.clone(),
            display_name: self.display_name.clone(),
            gender: if self.is_male {"male".to_string()} else {"female".to_string()},
            feast_day: format!("{:?}-{:?}",self.feast_day, self.feast_month),
        }
    }
}
