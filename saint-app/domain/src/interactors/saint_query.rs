use crate::boundaries;
use crate::boundaries::{SaintDbGateway, SaintQueryRequest, SaintQueryResponse};
use async_trait::async_trait;

pub struct SaintQueryInteractor<A: SaintDbGateway> {
    db_gateway: A,
}

#[async_trait]
impl<A> boundaries::SaintQueryInputBoundary for SaintQueryInteractor<A>
where
    A: SaintDbGateway + Sync + Send,
{
    async fn get_saint(&self, request: SaintQueryRequest) -> SaintQueryResponse {
        println!("saint mutation input boundary {}", request.name);
        let status: u16;
        if ((*self).db_gateway.exists_by_name(request.name.clone())).await {
            println!("user found");
            status = 200;
        } else {
            println!("user not found");
            status = 404;
        }
        SaintQueryResponse { status }
    }
}

impl<A> SaintQueryInteractor<A>
where
    A: SaintDbGateway + Sync + Send,
{
    pub fn new(db_gateway: A) -> Self {
        SaintQueryInteractor { db_gateway }
    }
}
