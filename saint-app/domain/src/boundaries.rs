use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait SaintQueryInputBoundary {
    async fn get_saint(&self, request: SaintQueryRequest) -> Option<SaintQueryResponse>;
    async fn get_saints(&self, request: SaintQueryRequest) -> SaintCollectionQueryResponse;
}

#[async_trait]
pub trait SaintMutationInputBoundary {
    async fn create_saint(
        &mut self,
        request: SaintMutationRequest,
    ) -> Result<SaintMutationResponse, SaintMutationError>;
    async fn update_saint(
        &mut self,
        request: SaintMutationRequest,
    ) -> Result<SaintMutationResponse, SaintMutationError>;
    async fn delete_saint(&self, request: SaintMutationRequest) -> Result<(), SaintMutationError>;
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
    pub id: Option<Uuid>,
    pub gender: Option<String>,
    pub display_name: Option<String>,
    pub offset: Option<u16>,
    pub count: Option<u16>,
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

pub struct SaintMutationResponse {
    pub id: Option<Uuid>,
    pub display_name: String,
    pub english_name: Option<String>,
    pub french_name: Option<String>,
    pub latin_name: Option<String>,
    pub vietnamese_name: String,
    pub gender: String,
    pub feast_day: String,
}
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

pub struct SaintCollectionQueryResponse {
    pub collection: Vec<SaintQueryResponse>,
}

pub struct SaintCollectionDbResponse {
    pub collection: Vec<SaintDbResponse>,
}

pub trait MutationOutputBoundary {}

#[async_trait]
pub trait SaintDbGateway {
    async fn find_by_id(&self, id: Uuid) -> Option<SaintDbResponse>;
    async fn exists_by_id(&self, id: Uuid) -> bool;
    async fn insert(&mut self, db_request: SaintDbRequest) -> Result<(), DbError>;
    async fn update(&mut self, db_request: SaintDbRequest) -> Result<(), DbError>;
    async fn delete(&self, id: Uuid) -> Result<(), DbError>;
    async fn get_saint_collection(
        &self,
        is_male: Option<bool>,
        display_name: Option<String>,
        offset: Option<u16>,
        count: Option<u16>,
    ) -> SaintCollectionDbResponse;
}

#[derive(Debug)]
pub enum SaintMutationError {
    UniqueConstraintViolationError(String),
    IdCollisionError,
    InvalidSaint,
    UnknownError(String),
    SaintNotFound,
}

#[derive(Debug)]
pub enum DbError {
    UniqueConstraintViolationError(String),
    UnknownError(String),
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
