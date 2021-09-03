use crate::boundaries;
use crate::boundaries::{
    SaintCollectionQueryResponse, SaintDbGateway, SaintDbResponse, SaintQueryRequest,
    SaintQueryResponse,
};
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
        println!(
            "saint query input boundary {}",
            request.id.unwrap().to_hyphenated()
        );

        if let Some(db_response) = ((*self).db_gateway.find_by_id(request.id.unwrap())).await {
            println!("saint found");
            return Some(db_response.to_saint_query_response());
        } else {
            println!("saint not found");
            return None;
        }
    }

    async fn get_saints(&self, request: SaintQueryRequest) -> SaintCollectionQueryResponse {
        println!("saint query input boundary");
        let is_male = request
            .gender
            .map(|value| if value.eq("MALE") { true } else { false });
        let display_name = request.display_name;
        let offset = request.offset;
        let count = request.count;

        let result =
            ((*self)
                .db_gateway
                .get_saint_collection(is_male, display_name, offset, count))
            .await;
        let collection = result
            .collection
            .into_iter()
            .map(|saint_db_response| saint_db_response.to_saint_query_response())
            .collect();
        SaintCollectionQueryResponse {
            collection: collection,
            has_more: result.has_more,
            total: result.total,
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
            gender: if self.is_male {
                "MALE".to_string()
            } else {
                "FEMALE".to_string()
            },
            feast_day: format!("{:02}-{:02}", self.feast_day, self.feast_month),
        }
    }
}
