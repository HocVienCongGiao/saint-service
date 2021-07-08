use lambda_http::http::header::{
    ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
    CONTENT_TYPE,
};
use lambda_http::http::HeaderValue;
use lambda_http::{handler, http, lambda_runtime, Body, Context, IntoResponse, Request, Response};
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

    let request = http::Request::builder()
        .uri("http://dev-sg.portal.hocvienconggiao.com/query-api/saint-service/saints/40e6215d-b5c6-4896-987c-f30f3678f608")
        .method("GET")
        .header("Content-Type", "application/json")
        .body(Body::Empty)
        .unwrap();

    let expected =
            "{\"id\":\"40e6215d-b5c6-4896-987c-f30f3678f608\",\"displayName\":\"Phêrô\",\"englishName\":\"Peter the Apostle\",\"frenchName\":\"saint Pierre\",\"latinName\":\"Simon Petrus\",\"vietnameseName\":\"Thánh Phêrô Tông đồ\",\"gender\":\"male\",\"feastDay\":\"29-6\"}"
        .into_response();

    let response = saint::saint(request, Context::default())
        .await
        .expect("expected Ok(_) value")
        .into_response();
    assert_eq!(response.body(), expected.body())
}

#[tokio::test]
async fn save_test() {
    initialise();
    println!("is it working?");

    let request = http::Request::builder()
        .uri("https://dev-sg.portal.hocvienconggiao.com/mutation-api/saint-service/saints")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(
            "
            {
                \"id\": \"3fa85f64-5717-4562-b3fc-2c963f66afa6\",
                \"displayName\": \"Anrê\",
                \"vietnameseName\": \"Anrê\",
                \"gender\": \"MALE\",
                \"feastDay\": \"31-12\"
            }
        ",
        ))
        .unwrap();

    let expected: Response<Body> = /*http::status::StatusCode::from_u16(200).unwrap()*/
        Response::builder()
            .header(CONTENT_TYPE, "application/json")
            .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .header(ACCESS_CONTROL_ALLOW_HEADERS, "*")
            .header(ACCESS_CONTROL_ALLOW_METHODS, "*")
            .status(200)
            .body(Body::Empty)
            .expect("unable to build http::Response")
        .into_response();

    let response = saint::saint(request, Context::default())
        .await
        .expect("expected Ok(_) value")
        .into_response();
    assert_eq!(response.status(), expected.status())
}
