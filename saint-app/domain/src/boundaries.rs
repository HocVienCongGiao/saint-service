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
    async fn delete_saint(
        &mut self,
        request: SaintMutationRequest,
    ) -> Result<(), SaintMutationError>;
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
    pub sort_request: Option<SaintSortRequest>,
    pub offset: Option<i64>,
    pub count: Option<i64>,
}

pub struct SaintSortRequest {
    pub sort_criteria: Vec<SaintSortCriteriaRequest>,
}

pub struct SaintSortCriteriaRequest {
    pub field: SaintSortFieldRequest,
    pub direction: SortDirectionRequest,
}

pub enum SaintSortFieldRequest {
    DisplayName,
    VietnameseName,
    EnglishName,
    FeastDay,
    FeastMonth,
}

pub enum SortDirectionRequest {
    ASC,
    DESC,
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

pub struct SaintSortDbRequest {
    pub sort_criteria: Vec<SaintSortCriteriaDbRequest>,
}

pub struct SaintSortCriteriaDbRequest {
    pub field: SaintSortFieldDbRequest,
    pub direction: SortDirectionDbRequest,
}

pub enum SaintSortFieldDbRequest {
    DisplayName,
    VietnameseName,
    EnglishName,
    FeastDay,
    FeastMonth,
}

pub enum SortDirectionDbRequest {
    ASC,
    DESC,
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
    pub has_more: Option<bool>,
    pub total: i64,
}

pub struct SaintCollectionDbResponse {
    pub collection: Vec<SaintDbResponse>,
    pub has_more: Option<bool>,
    pub total: i64,
}

pub trait MutationOutputBoundary {}

#[async_trait]
pub trait SaintDbGateway {
    async fn get_saint_by_id(&self, id: Uuid) -> Option<SaintDbResponse>;
    async fn exists_by_id(&self, id: Uuid) -> bool;
    async fn insert(&mut self, db_request: SaintDbRequest) -> Result<(), DbError>;
    async fn update(&mut self, db_request: SaintDbRequest) -> Result<(), DbError>;
    async fn delete(&mut self, id: Uuid) -> Result<(), DbError>;
    async fn get_saint_collection(
        &self,
        is_male: Option<bool>,
        display_name: Option<String>,
        sort_db_request: Option<SaintSortDbRequest>,
        offset: Option<i64>,
        count: Option<i64>,
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
