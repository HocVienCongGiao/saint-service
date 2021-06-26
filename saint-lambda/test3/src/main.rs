use lambda_http::http::header::{
    ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
    CONTENT_TYPE,
};
use lambda_http::http::HeaderValue;
use lambda_http::{handler, lambda, Body, Context, IntoResponse, Request, Response};
use serde_json::json;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

use test3::test3;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(test3)).await?;
    Ok(())
}
