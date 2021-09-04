use crate::boundaries;
use crate::boundaries::{
    SaintCollectionQueryResponse, SaintDbGateway, SaintDbResponse, SaintQueryRequest,
    SaintQueryResponse, SaintSortCriteriaDbRequest, SaintSortCriteriaRequest, SaintSortDbRequest,
    SaintSortFieldDbRequest, SaintSortFieldRequest, SaintSortRequest, SortDirectionDbRequest,
    SortDirectionRequest,
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

        if let Some(db_response) = ((*self).db_gateway.get_saint_by_id(request.id.unwrap())).await {
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
        let vietnamese_name = request.vietnamese_name;
        let english_name = request.english_name;
        let feast_day = request.feast_day;
        let feast_month = request.feast_month;
        let offset = request.offset;
        let count = request.count;
        let sort_request = request.sort_request;

        let sort_db_request: Option<SaintSortDbRequest>;
        if let Some(sort_request) = sort_request {
            sort_db_request = Option::from(sort_request.to_db_request());
        } else {
            sort_db_request = None
        }

        let result = ((*self).db_gateway.get_saint_collection(
            is_male,
            display_name,
            vietnamese_name,
            english_name,
            feast_day,
            feast_month,
            sort_db_request,
            offset,
            count,
        ))
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

impl SaintSortRequest {
    fn to_db_request(&self) -> SaintSortDbRequest {
        let sort_criteria_db_request = self
            .sort_criteria
            .iter()
            .map(|criterion| criterion.to_db_request())
            .collect();
        SaintSortDbRequest {
            sort_criteria: sort_criteria_db_request,
        }
    }
}

impl SaintSortCriteriaRequest {
    fn to_db_request(&self) -> SaintSortCriteriaDbRequest {
        let field = &self.field;
        let direction = &self.direction;

        SaintSortCriteriaDbRequest {
            field: match field {
                SaintSortFieldRequest::DisplayName => SaintSortFieldDbRequest::DisplayName,
                SaintSortFieldRequest::EnglishName => SaintSortFieldDbRequest::EnglishName,
                SaintSortFieldRequest::VietnameseName => SaintSortFieldDbRequest::VietnameseName,
                SaintSortFieldRequest::FeastDay => SaintSortFieldDbRequest::FeastDay,
                SaintSortFieldRequest::FeastMonth => SaintSortFieldDbRequest::FeastMonth,
            },
            direction: match direction {
                SortDirectionRequest::ASC => SortDirectionDbRequest::ASC,
                SortDirectionRequest::DESC => SortDirectionDbRequest::DESC,
            },
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
