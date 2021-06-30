use lambda_http::http::header::{
    ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
    CONTENT_TYPE,
};
use lambda_http::http::HeaderValue;
use lambda_http::{handler, lambda_runtime, Body, Context, IntoResponse, Request, Response};
use saint::saint;
use serde_json::json;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

// use pg_embed::postgres::PgEmbed;
use std::path::PathBuf;
use std::sync::Once;

static INIT: Once = Once::new();

fn initialise() {
    INIT.call_once(|| {
        let my_path = PathBuf::new().join(".env.test");
        dotenv::from_path(my_path.as_path()).ok();
        // println!("testing env {}", std::env::var("HELLO").unwrap());
    });
}

#[tokio::test]
async fn integration_works() {
    initialise();
    println!("is it working?");
    let request = Request::default();
    /*let expected = json!(
        {"id":"40e6215d-b5c6-4896-987c-f30f3678f608","englishName":"Peter the Apostle",
        "frenchName":"saint Pierre","latinName":"Simon Petrus","vietnameseName":"Thánh Phêrô Tông đồ",
        "gender":"male","feastDay":"29-6"}
    )
    .into_response();*/

    let expected =
            "{\"id\":\"40e6215d-b5c6-4896-987c-f30f3678f608\",\"englishName\":\"Peter the Apostle\",\"frenchName\":\"saint Pierre\",\"latinName\":\"Simon Petrus\",\"vietnameseName\":\"Thánh Phêrô Tông đồ\",\"gender\":\"male\",\"feastDay\":\"29-6\"}"
        .into_response();

    let response = saint::saint(request, Context::default())
        .await
        .expect("expected Ok(_) value")
        .into_response();
    assert_eq!(response.body(), expected.body())
}
