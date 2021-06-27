use async_trait::async_trait;
use tokio_postgres::Client;
use uuid::Uuid;

mod mutation;
mod query;

use domain::boundaries::SaintDbResponse;

pub struct SaintRepository {
    pub client: Client,
}

#[async_trait]
impl domain::boundaries::SaintDbGateway for SaintRepository {
    async fn find_by_id(&self, id: Uuid) -> Option<SaintDbResponse> {
        let result = query::find_one_by_id(&(*self).client, id.clone()).await;
        println!("second block_on for row");
        if result.is_err() {
            return None;
        }
        let row = result.unwrap();

        let id: Option<Uuid>;
        let english_name: Option<String>;
        let french_name: Option<String>;
        let latin_name: Option<String>;

        if let Some(value) = row.get("id") {
            id = Some(value)
        } else {
            id = None
        };
        if let Some(value) = row.get("english_name") {
            english_name = Some(value)
        } else {
            english_name = None
        };
        if let Some(value) = row.get("french_name") {
            french_name = Some(value)
        } else {
            french_name = None
        };
        if let Some(value) = row.get("latin_name") {
            latin_name = Some(value)
        } else {
            latin_name = None
        };
        let vietnamese_name: String = row.get("vietnamese_name");
        let display_name: String = row.get("display_name");
        let is_male: bool = row.get("is_male");
        let feast_day: i16 = row.get("feast_day");
        let feast_month: i16 = row.get("feast_month");

        Some(SaintDbResponse {
            id,
            english_name,
            french_name,
            latin_name,
            vietnamese_name,
            display_name,
            is_male,
            feast_day,
            feast_month,
        })
    }

    async fn insert(&self, name: String, country: String) -> bool {
        let result = query::save(&(*self).client, name.clone(), country.clone()).await;
        return if result.is_err() { false } else { true };
    }

    async fn exists_by_id(&self, name: String) -> bool {
        todo!()
    }
}
