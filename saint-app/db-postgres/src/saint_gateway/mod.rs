use async_trait::async_trait;
use tokio_postgres::Client;

mod mutation;
mod query;

pub struct SaintRepository {
    pub client: Client,
}

#[async_trait]
impl domain::boundaries::SaintDbGateway for SaintRepository {
    async fn exists_by_name(&self, name: String) -> bool {
        let result = query::find_one_by_name(&(*self).client, name.clone()).await;
        println!("second block_on for row");
        if result.is_err() {
            return false;
        }
        let row = result.unwrap();
        let name_found: String = row.get("name");
        println!("ROW IS {}", name);
        name_found == name
    }

    async fn insert(&self, name: String, country: String) -> bool {
        let result = query::save(&(*self).client, name.clone(), country.clone()).await;
        return if result.is_err() { false } else { true };
    }
}
