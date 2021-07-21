use hvcg_biography_openapi_saint::models::Saint;
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
            "{\"id\":\"40e6215d-b5c6-4896-987c-f30f3678f608\",\"displayName\":\"Phêrô\",\"englishName\":\"Peter the Apostle\",\"frenchName\":\"saint Pierre\",\"latinName\":\"Simon Petrus\",\"vietnameseName\":\"Thánh Phêrô Tông đồ\",\"gender\":\"MALE\",\"feastDay\":\"29-06\"}"
        .into_response();

    let response = saint::saint(request, Context::default())
        .await
        .expect("expected Ok(_) value")
        .into_response();
    assert_eq!(response.body(), expected.body())
}

#[tokio::test]
async fn test_get_saints() {
    initialise();
    println!("is it working?");

    let request = http::Request::builder()
        .uri("https://dev-sg.portal.hocvienconggiao.com/query-api/saint-service/saints?count=20")
        .method("GET")
        .header("Content-Type", "application/json")
        .body(Body::Empty)
        .unwrap();

    let expected =
            "[{\"id\":\"40e6215d-b5c6-4896-987c-f30f3678f608\",\"displayName\":\"Phêrô\",\"englishName\":\"Peter the Apostle\",\"frenchName\":\"saint Pierre\",\"latinName\":\"Simon Petrus\",\"vietnameseName\":\"Thánh Phêrô Tông đồ\",\"gender\":\"MALE\",\"feastDay\":\"29-06\"}]"
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

    let saint_request = Saint {
        id: None,
        display_name: "Anrê".to_string(),
        english_name: None,
        french_name: None,
        latin_name: None,
        vietnamese_name: "Anrê".to_string(),
        gender: "MALE".to_string(),
        feast_day: "31-12".to_string(),
    };

    let serialized_saint = serde_json::to_string(&saint_request).unwrap();

    let request = http::Request::builder()
        .uri("https://dev-sg.portal.hocvienconggiao.com/mutation-api/saint-service/saints")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(serialized_saint))
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

    assert_eq!(response.status(), expected.status());

    let empty_saint = Saint {
        id: None,
        display_name: "".to_string(),
        english_name: None,
        french_name: None,
        latin_name: None,
        vietnamese_name: "".to_string(),
        gender: "".to_string(),
        feast_day: "".to_string(),
    };
    let deserialized_saint: Saint;
    if let Body::Text(saint_obj) = response.body() {
        deserialized_saint =
            serde_json::from_str(saint_obj).expect("Unable deserialise response body");
    } else {
        deserialized_saint = empty_saint;
    }
    let expected_saint = Saint {
        id: deserialized_saint.id,
        display_name: "Anrê".to_string(),
        english_name: None,
        french_name: None,
        latin_name: None,
        vietnamese_name: "Anrê".to_string(),
        gender: "MALE".to_string(),
        feast_day: "31-12".to_string(),
    };
    assert_eq!(deserialized_saint, expected_saint);
}

#[tokio::test]
async fn put_test() {
    initialise();
    println!("is it working?");

    println!("---test put method with saint not existing---");
    let empty_saint = Saint {
        id: None,
        display_name: "".to_string(),
        english_name: None,
        french_name: None,
        latin_name: None,
        vietnamese_name: "".to_string(),
        gender: "".to_string(),
        feast_day: "".to_string(),
    };

    let saint_request = Saint {
        id: None,
        display_name: "test".to_string(),
        english_name: None,
        french_name: None,
        latin_name: None,
        vietnamese_name: "test".to_string(),
        gender: "MALE".to_string(),
        feast_day: "01-01".to_string(),
    };

    let serialized_saint = serde_json::to_string(&saint_request).unwrap();

    let request = http::Request::builder()
        .uri("https://dev-sg.portal.hocvienconggiao.com/mutation-api/saint-service/saints/00000000-0000-0000-0000-000000000000")
        .method("PUT")
        .header("Content-Type", "application/json")
        .body(Body::from(serialized_saint))
        .unwrap();

    let response = saint::saint(request, Context::default())
        .await
        .expect("expected Ok(_) value")
        .into_response();

    let deserialized_saint: Saint;
    if let Body::Text(saint_obj) = response.body() {
        deserialized_saint =
            serde_json::from_str(saint_obj).expect("Unable deserialise response body");
    } else {
        deserialized_saint = empty_saint;
    }

    let save_id = deserialized_saint.id;
    let expected_saint = Saint {
        id: save_id,
        display_name: "test".to_string(),
        english_name: None,
        french_name: None,
        latin_name: None,
        vietnamese_name: "test".to_string(),
        gender: "MALE".to_string(),
        feast_day: "01-01".to_string(),
    };
    assert_eq!(deserialized_saint, expected_saint);

    println!("---test put method with saint existing---");
    let empty_saint = Saint {
        id: None,
        display_name: "".to_string(),
        english_name: None,
        french_name: None,
        latin_name: None,
        vietnamese_name: "".to_string(),
        gender: "".to_string(),
        feast_day: "".to_string(),
    };
    let saint_request = Saint {
        id: None,
        display_name: "update".to_string(),
        english_name: Some("update".to_string()),
        french_name: Some("update".to_string()),
        latin_name: Some("update".to_string()),
        vietnamese_name: "update".to_string(),
        gender: "MALE".to_string(),
        feast_day: "01-01".to_string(),
    };
    let serialized_saint = serde_json::to_string(&saint_request).unwrap();
    let uri = format!(
        "http://dev-sg.portal.hocvienconggiao.com/mutation-api/saint-service/saints/{}",
        save_id.unwrap().to_hyphenated()
    );
    let request = http::Request::builder()
        .uri(uri)
        .method("PUT")
        .header("Content-Type", "application/json")
        .body(Body::from(serialized_saint))
        .unwrap();

    let response = saint::saint(request, Context::default())
        .await
        .expect("expected Ok(_) value")
        .into_response();

    let deserialized_saint: Saint;
    if let Body::Text(saint_obj) = response.body() {
        deserialized_saint =
            serde_json::from_str(saint_obj).expect("Unable deserialise response body");
    } else {
        deserialized_saint = empty_saint;
    }
    let expected_saint = Saint {
        id: save_id,
        display_name: "update".to_string(),
        english_name: Some("update".to_string()),
        french_name: Some("update".to_string()),
        latin_name: Some("update".to_string()),
        vietnamese_name: "update".to_string(),
        gender: "MALE".to_string(),
        feast_day: "01-01".to_string(),
    };
    assert_eq!(deserialized_saint, expected_saint);
}

#[tokio::test]
async fn delete_test() {
    initialise();
    println!("is it working?");

    println!("---Creating Saint---");
    let saint_request = Saint {
        id: None,
        display_name: "delete_test".to_string(),
        english_name: None,
        french_name: None,
        latin_name: None,
        vietnamese_name: "delete_test".to_string(),
        gender: "MALE".to_string(),
        feast_day: "01-01".to_string(),
    };
    let serialized_saint = serde_json::to_string(&saint_request).unwrap();
    let request = http::Request::builder()
        .uri("https://dev-sg.portal.hocvienconggiao.com/mutation-api/saint-service/saints")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(serialized_saint))
        .unwrap();
    let response = saint::saint(request, Context::default())
        .await
        .expect("expected Ok(_) value")
        .into_response();

    assert_eq!(response.status(), 200);

    let empty_saint = Saint {
        id: None,
        display_name: "".to_string(),
        english_name: None,
        french_name: None,
        latin_name: None,
        vietnamese_name: "".to_string(),
        gender: "".to_string(),
        feast_day: "".to_string(),
    };
    let deserialized_saint: Saint;
    if let Body::Text(saint_obj) = response.body() {
        deserialized_saint =
            serde_json::from_str(saint_obj).expect("Unable deserialise response body");
    } else {
        deserialized_saint = empty_saint;
    }
    let save_id = deserialized_saint.id;

    println!("---Deleting Saint---");
    let uri = format!(
        "http://dev-sg.portal.hocvienconggiao.com/mutation-api/saint-service/saints/{}",
        save_id.unwrap().to_hyphenated()
    );
    let request = http::Request::builder()
        .uri(uri)
        .method("DELETE")
        .header("Content-Type", "application/json")
        .body(Body::Empty)
        .unwrap();
    let response = saint::saint(request, Context::default())
        .await
        .expect("expected Ok(_) value")
        .into_response();

    assert_eq!(response.status(), 204);

    println!("---Try getting Saint after deleting---");
    let uri = format!(
        "http://dev-sg.portal.hocvienconggiao.com/query-api/saint-service/saints/{}",
        save_id.unwrap().to_hyphenated()
    );
    let request = http::Request::builder()
        .uri(uri)
        .method("GET")
        .header("Content-Type", "application/json")
        .body(Body::Empty)
        .unwrap();
    let response = saint::saint(request, Context::default())
        .await
        .expect("expected Ok(_) value")
        .into_response();

    assert_eq!(response.status(), 404)
}

