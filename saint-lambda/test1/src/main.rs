use lambda_http::{handler, lambda, Context, IntoResponse, Request};
use serde_json::json;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(test1)).await?;
    Ok(())
}

async fn test1(_: Request, _: Context) -> Result<impl IntoResponse, Error> {
    // controller::openapi::test1::create_test1();
    let saint = hvcg_biography_openapi_saint::models::Saint {
        id: None,
        english_name: None,
        french_name: None,
        latin_name: None,
        vietnamese_name: "Thánh Phêrô Tông đồ".to_string(),
        gender: "male".to_string(),
        feast_day: "29-6".to_string(),
    };
    // `serde_json::Values` impl `IntoResponse` by default
    // creating an application/json response
    Ok(json!(saint))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test1_handles() {
        let my_path = PathBuf::new().join(".env.test");
        dotenv::from_path(my_path.as_path()).ok();
        // let test = controller::get_test1().await;
        // println!("hello {}", test.status);
        let request = Request::default();
        let expected = json!({
            "vietnameseName": "Thánh Phêrô Tông đồ", "gender": "male", "feastDay": "29-6",
        })
        .into_response();
        let response = test1(request, Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body())
    }
}
