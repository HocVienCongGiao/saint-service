pub fn test_func() {
    println!("hello");
}

pub mod boundaries;
mod entity;
pub mod interactors;

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use crate::boundaries::{
        SaintDbGateway, SaintMutationInputBoundary, SaintMutationRequest, SaintDbResponse,
    };
    use async_trait::async_trait;

    struct SaintDbGatewayStub {}

    #[async_trait]
    impl SaintDbGateway for SaintDbGatewayStub {
        async fn exists_by_id(&self, name: String) -> bool {
            if name == "existing" {
                return true;
            }
            false
        }

        async fn insert(&self, name: String, country: String) -> bool {
            todo!()
        }

        async fn find_by_id(&self, id: Uuid) -> Option<SaintDbResponse> {
            todo!()
        }
    }

    #[test]
    fn it_works() {
        let saint_mutator =
            crate::interactors::saint_mutation::SaintMutationInteractor::new(Box::new(
                SaintDbGatewayStub {},
            ));
        saint_mutator.create_saint(SaintMutationRequest {
            name: "existing".to_string(),
        });
        saint_mutator.create_saint(SaintMutationRequest {
            name: "new".to_string(),
        });
        assert_eq!(2 + 2, 4);
    }
}
