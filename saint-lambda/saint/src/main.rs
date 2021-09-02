use lambda_http::{handler, lambda_runtime};

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

use saint::saint;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(saint)).await?;
    Ok(())
}
