pub use domain::boundaries::{SaintMutationResponse, SaintQueryResponse, SaintCollectionQueryResponse};
pub use hvcg_biography_openapi_saint::models::{Saint, SaintCollection};

pub fn create_saint() {
    println!("Creating Saint in Controller OpenApi saint.rs")
}

impl ToOpenApi<Saint> for SaintQueryResponse {
    fn to_openapi(self) -> hvcg_biography_openapi_saint::models::Saint {
        Saint {
            id: self.id,
            display_name: self.display_name,
            english_name: self.english_name,
            french_name: self.french_name,
            latin_name: self.latin_name,
            vietnamese_name: self.vietnamese_name,
            gender: self.gender,
            feast_day: self.feast_day,
        }
    }
}

impl ToOpenApi<Saint> for SaintMutationResponse {
    fn to_openapi(self) -> hvcg_biography_openapi_saint::models::Saint {
        Saint {
            id: self.id,
            display_name: self.display_name,
            english_name: self.english_name,
            french_name: self.french_name,
            latin_name: self.latin_name,
            vietnamese_name: self.vietnamese_name,
            gender: self.gender,
            feast_day: self.feast_day,
        }
    }
}

impl ToOpenApi<SaintCollection> for SaintCollectionQueryResponse {
    fn to_openapi(self) -> SaintCollection {
        let collection = (self.collection
                        .into_iter()
                        .map(|saint_query_response| saint_query_response.to_openapi())
                        .collect::<Vec<Saint>>()).to_vec();
        SaintCollection {
            saints: Some(collection),
            has_more: self.has_more,
        }
    }
}

pub trait ToOpenApi<T> {
    fn to_openapi(self) -> T;
}
