use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait SaintQueryInputBoundary {
    async fn get_saint(&self, request: SaintQueryRequest) -> Option<SaintQueryResponse>;
}

pub trait SaintMutationInputBoundary {
    fn create_saint(&self, request: SaintMutationRequest) -> Option<SaintMutationResponse>;
}

pub struct SaintMutationRequest {
    pub id: Option<Uuid>,
    pub display_name: Option<String>,
    pub english_name: Option<String>,
    pub french_name: Option<String>,
    pub latin_name: Option<String>,
    pub vietnamese_name: Option<String>,
    pub gender: Option<String>,
    pub feast_day: Option<String>,
}
pub struct SaintQueryRequest {
    pub id: Uuid,
}
pub struct SaintDbRequest {
    pub id: Option<Uuid>,
    pub display_name: Option<String>,
    pub english_name: Option<String>,
    pub french_name: Option<String>,
    pub latin_name: Option<String>,
    pub vietnamese_name: Option<String>,
    pub is_male: Option<bool>,
    pub feast_day: Option<i16>,
    pub feast_month: Option<i16>,
}

pub struct SaintMutationResponse {}
pub struct SaintQueryResponse {
    pub id: Option<Uuid>,
    pub display_name: String,
    pub english_name: Option<String>,
    pub french_name: Option<String>,
    pub latin_name: Option<String>,
    pub vietnamese_name: String,
    pub gender: String,
    pub feast_day: String,
}
pub struct SaintDbResponse {
    pub id: Option<Uuid>,
    pub display_name: String,
    pub english_name: Option<String>,
    pub french_name: Option<String>,
    pub latin_name: Option<String>,
    pub vietnamese_name: String,
    pub is_male: bool,
    pub feast_day: i16,
    pub feast_month: i16,
}

pub trait MutationOutputBoundary {}

#[async_trait]
pub trait SaintDbGateway {
    async fn find_by_id(&self, id: Uuid) -> Option<SaintDbResponse>;
    async fn exists_by_id(&self, id: Uuid) -> bool;
    async fn insert(&self, db_request: SaintDbRequest) -> bool;
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
