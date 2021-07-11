pub use domain::boundaries::{SaintMutationResponse, SaintQueryResponse};
pub use hvcg_biography_openapi_saint::models::Saint;

pub fn create_saint() {
    println!("Creating Saint in Controller OpenApi saint.rs")
}

impl ToOpenApi<Saint> for SaintQueryResponse {
    fn to_openapi(&self) -> hvcg_biography_openapi_saint::models::Saint {
        Saint {
            id: self.id.clone(),
            display_name: self.display_name.clone(),
            english_name: self.english_name.clone(),
            french_name: self.french_name.clone(),
            latin_name: self.latin_name.clone(),
            vietnamese_name: self.vietnamese_name.clone(),
            gender: self.gender.clone(),
            feast_day: self.feast_day.clone(),
        }
    }
}

impl ToOpenApi<Saint> for SaintMutationResponse {
    fn to_openapi(&self) -> hvcg_biography_openapi_saint::models::Saint {
        Saint {
            id: self.id.clone(),
            display_name: self.display_name.clone(),
            english_name: self.english_name.clone(),
            french_name: self.french_name.clone(),
            latin_name: self.latin_name.clone(),
            vietnamese_name: self.vietnamese_name.clone(),
            gender: self.gender.clone(),
            feast_day: self.feast_day.clone(),
        }
    }
}

pub trait ToOpenApi<T> {
    fn to_openapi(&self) -> T;
}
