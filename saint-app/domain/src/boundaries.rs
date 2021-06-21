use async_trait::async_trait;

#[async_trait]
pub trait SaintQueryInputBoundary {
    async fn get_saint(&self, request: SaintQueryRequest) -> SaintQueryResponse;
}

pub trait SaintMutationInputBoundary {
    fn create_saint(&self, request: SaintMutationRequest) -> SaintMutationResponse;
}

pub struct SaintMutationRequest {
    pub name: String,
}
pub struct SaintQueryRequest {
    pub name: String,
}

pub struct SaintMutationResponse {}
pub struct SaintQueryResponse {
    pub status: u16,
}

pub trait MutationOutputBoundary {}

#[async_trait]
pub trait SaintDbGateway {
    async fn exists_by_name(&self, name: String) -> bool;
    async fn insert(&self, name: String, country: String) -> bool;
}

// CommonUser
// CommonUserFactory
// JpaUser
// JpaUserRepository
// User
// UserDataMapper
// UserDsRequestModel
// UserFactory
// UserInputBoundary
// UserPresenter
// UserRegisterController
// UserRegisterDsGateway
// UserRegisterInteractor
// UserRequestModel
// UserResponseFormatter
// UserResponseModel
