use domain::boundaries::{
    SaintMutationError, SaintSortCriteriaRequest, SaintSortFieldRequest, SaintSortRequest,
    SortDirectionRequest,
};
use hvcg_biography_openapi_saint::models::{Saint, SaintCollection, SaintSortCriteria};
use jsonwebtoken::TokenData;
use lambda_http::http::header::{
    ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
    CONTENT_TYPE,
};
use lambda_http::http::{method, HeaderValue};
use lambda_http::{Body, Context, IntoResponse, Request, RequestExt, Response};
use serde::{Deserialize, Serialize};

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[derive(Deserialize, Serialize)]
struct TokenPayload {
    // Despite the struct field being named `username`, it is going to come
    // from a JSON field called `cognito:username`.
    #[serde(rename(deserialize = "cognito:username"))]
    username: String,
    #[serde(rename(deserialize = "cognito:groups"))]
    groups: Vec<String>,
}

/**
{
   "sub":"fd9a7af8-fa76-4b86-af3d-d9634ef52374",
   "aud":"1rav411nccnp73htopbhml8s61",
   "cognito:groups":[
      "OperatorGroup"
   ],
   "event_id":"9645d622-3f4b-42b7-9b4d-71d35da9256d",
   "token_use":"id",
   "auth_time":1623934926,
   "iss":"https:\/\/cognito-idp.ap-southeast-1.amazonaws.com\/ap-southeast-1_9QWSYGzXk",
   "phone_number_verified":true,
   "cognito:username":"dev-operator",
   "phone_number":"+84369140916",
   "exp":1623938526,
   "iat":1623934926
}
*/

pub struct SaintQuery {
    gender: Option<String>,
    display_name: Option<String>,
    vietnamese_name: Option<String>,
    english_name: Option<String>,
    feast_day: Option<String>, // TODO Implement filtering
    sort_criteria: Vec<String>,
    offset: Option<i64>,
    count: Option<i64>,
}

pub fn get_query_from_request(req: &Request) -> SaintQuery {
    let query = req.query_string_parameters();
    SaintQuery {
        gender: query.get("gender").map(|str| str.to_string()),
        display_name: query.get("displayName").map(|str| str.to_string()),
        vietnamese_name: query.get("vietnameseName").map(|str| str.to_string()),
        english_name: query.get("englishName").map(|str| str.to_string()),
        feast_day: query.get("feastDay").map(|str| str.to_string()),
        sort_criteria: query
            .get_all("sorts")
            .unwrap_or_default()
            .iter()
            .map(|criterion| criterion.to_string())
            .collect(),
        offset: query.get("offset").map(|str| str.parse().unwrap()),
        count: query.get("count").map(|str| str.parse().unwrap()),
    }
}

pub fn get_id_from_request(req: &Request) -> Option<uuid::Uuid> {
    let path_parameters = req.path_parameters();
    let id_param = path_parameters.get("id");
    if let Some(id) = id_param {
        println!("id found");
        Some(uuid::Uuid::parse_str(id).unwrap())
    } else {
        println!("id not found");
        None
    }
}

pub async fn saint(req: Request, ctx: Context) -> Result<impl IntoResponse, Error> {
    println!("Request {:?}", req);
    println!("ctx.deadline {}", ctx.deadline);
    println!("path_parameters {:?}", req.path_parameters());
    println!(
        "query_string_parameters {:?}",
        req.query_string_parameters()
    );
    println!("Request Method {:?}", req.method());

    if req.method() == method::Method::OPTIONS {
        return Ok(Response::builder()
            .header(CONTENT_TYPE, "application/json")
            .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .header(ACCESS_CONTROL_ALLOW_HEADERS, "*")
            .header(ACCESS_CONTROL_ALLOW_METHODS, "*")
            .status(200)
            .body(Body::Empty)
            .expect("unable to build http::Response"));
    }

    // `serde_json::Values` impl `IntoResponse` by default
    let default_header_value = HeaderValue::from_str("Bearer eyJraWQiOiJaTGpneG41SStaZEpldnJRb0lpMTZEWEZoRHI4eG9UbVZ2b2ZuVm5vb3RFPSIsImFsZyI6IlJTMjU2In0.eyJzdWIiOiJmZDlhN2FmOC1mYTc2LTRiODYtYWYzZC1kOTYzNGVmNTIzNzQiLCJhdWQiOiIxcmF2NDExbmNjbnA3M2h0b3BiaG1sOHM2MSIsImNvZ25pdG86Z3JvdXBzIjpbIk9wZXJhdG9yR3JvdXAiXSwiZXZlbnRfaWQiOiI5NjQ1ZDYyMi0zZjRiLTQyYjctOWI0ZC03MWQzNWRhOTI1NmQiLCJ0b2tlbl91c2UiOiJpZCIsImF1dGhfdGltZSI6MTYyMzkzNDkyNiwiaXNzIjoiaHR0cHM6XC9cL2NvZ25pdG8taWRwLmFwLXNvdXRoZWFzdC0xLmFtYXpvbmF3cy5jb21cL2FwLXNvdXRoZWFzdC0xXzlRV1NZR3pYayIsInBob25lX251bWJlcl92ZXJpZmllZCI6dHJ1ZSwiY29nbml0bzp1c2VybmFtZSI6ImRldi1vcGVyYXRvciIsInBob25lX251bWJlciI6Iis4NDM2OTE0MDkxNiIsImV4cCI6MTYyMzk0ODAwOCwiaWF0IjoxNjIzOTQ0NDA4fQ.ml3N8J7uw4rbQOneEdnmQW6OwsAY6ycmp5PIrKGZKF3yWQn0oQECIhF2Q_jjWOjWPikpUQEy5IKgghiJLukgKo7q-T4tUauPG3GJxoSGQkfVcglkNu8nZTu7ioxXzlQAWsXLakgkH40mGzI6kl2hkEhRQh_lWGrT7TqDP2yVTsDMKEGJBdtcb-kFCnYHfn9FMoCyVGo4K3tSrkeGno7bzwO_XpFtZRhv9Qs4OtfESXARYCP3St69hyf4JuAop6-Zb38FPWcp6rnpRG3BF64YPGqo0J0MAyWVz_Du7Pk3-H5uZqqrr6iHKoPwoabPPlZxJ3JGdifVt_I54SwTbelbzw").unwrap();
    let auth_header_value = req
        .headers()
        .get("authorization")
        .unwrap_or(&default_header_value);
    let auth_header_str = auth_header_value.to_str().unwrap();
    let username: String;
    let groups: Vec<String>;
    if auth_header_str != "anonymous12" {
        let jwt_token = &auth_header_str.to_string()[7..];
        let token_data: TokenData<TokenPayload> =
            jsonwebtoken::dangerous_insecure_decode(jwt_token).unwrap();
        let token_payload = token_data.claims;
        username = token_payload.username;
        groups = token_payload.groups;
        println!("Groups include {:?}", groups);
    } else {
        username = String::from("anonymous");
    }
    println!("token username {}", username);
    println!("auth_header is {}", auth_header_str);
    println!("req.headers() is {:?}", req.headers());

    let saint_response: Option<controller::openapi::saint::Saint>;
    let saint_collection: Option<SaintCollection>;
    let mut is_get_saints = false;
    let status_code: u16;
    match *req.method() {
        method::Method::GET => {
            if let Some(id) = get_id_from_request(&req) {
                saint_collection = None;
                saint_response = controller::get_saint(id).await;
                if saint_response.is_none() {
                    status_code = 404;
                } else {
                    status_code = 200;
                }
            } else {
                let query = get_query_from_request(&req);
                let gender: Option<String> = query.gender;
                let display_name: Option<String> = query.display_name;
                let vietnamese_name = query.vietnamese_name;
                let english_name = query.english_name;

                // TODO handle searching
                let feast_day: Option<i16> = None;
                let feast_month: Option<i16> = None;

                let sort_request: Option<SaintSortRequest>;
                let sort_criteria_dto = if query.sort_criteria.is_empty() {
                    None
                } else {
                    Option::from(query.sort_criteria)
                };

                if let Some(sort_criteria_dto) = sort_criteria_dto {
                    let mut sort_criteria = Vec::new();
                    sort_criteria_dto.iter().for_each(|criterion| {
                        let s: SaintSortCriteria = criterion.parse().unwrap();
                        let sort_criteria_request = match s {
                            SaintSortCriteria::DISPLAY_NAME_ASC => build_sort_criteria_request(
                                SaintSortFieldRequest::DisplayName,
                                SortDirectionRequest::ASC,
                            ),
                            SaintSortCriteria::DISPLAY_NAME_DESC => build_sort_criteria_request(
                                SaintSortFieldRequest::DisplayName,
                                SortDirectionRequest::DESC,
                            ),
                            SaintSortCriteria::VIETNAMESE_NAME_ASC => build_sort_criteria_request(
                                SaintSortFieldRequest::EnglishName,
                                SortDirectionRequest::ASC,
                            ),
                            SaintSortCriteria::VIETNAMESE_NAME_DESC => build_sort_criteria_request(
                                SaintSortFieldRequest::EnglishName,
                                SortDirectionRequest::DESC,
                            ),
                            SaintSortCriteria::ENGLISH_NAME_ASC => build_sort_criteria_request(
                                SaintSortFieldRequest::EnglishName,
                                SortDirectionRequest::ASC,
                            ),
                            SaintSortCriteria::ENGLISH_NAME_DESC => build_sort_criteria_request(
                                SaintSortFieldRequest::EnglishName,
                                SortDirectionRequest::DESC,
                            ),
                            SaintSortCriteria::FEAST_DAY_ASC => build_sort_criteria_request(
                                SaintSortFieldRequest::FeastDay,
                                SortDirectionRequest::ASC,
                            ),
                            SaintSortCriteria::FEAST_DAY_DESC => build_sort_criteria_request(
                                SaintSortFieldRequest::FeastDay,
                                SortDirectionRequest::DESC,
                            ),
                        };

                        let sort_field = sort_criteria_request.field.clone();
                        let sort_direction = sort_criteria_request.direction.clone();
                        if SaintSortFieldRequest::FeastDay == sort_field {
                            sort_criteria.push(build_sort_criteria_request(
                                SaintSortFieldRequest::FeastMonth,
                                sort_direction,
                            ))
                        }
                        sort_criteria.push(sort_criteria_request);
                    });

                    sort_request = Option::from(SaintSortRequest { sort_criteria });
                } else {
                    sort_request = None;
                }

                let offset: Option<i64> = query.offset;
                let count: Option<i64> = query.count;
                saint_collection = Some(
                    controller::get_saints(
                        gender,
                        display_name,
                        vietnamese_name,
                        english_name,
                        feast_day,
                        feast_month,
                        sort_request,
                        offset,
                        count,
                    )
                    .await,
                );
                is_get_saints = true;
                saint_response = None;
                status_code = 200;
            }
        }
        method::Method::POST => {
            saint_collection = None;
            if let Some(value) = req.payload().unwrap_or(None) {
                let lambda_saint_request: Saint = value;
                let serialized_saint = serde_json::to_string(&lambda_saint_request).unwrap();
                println!("saint_obj: {}", serialized_saint);
                let result = controller::create_saint(&lambda_saint_request).await;
                match result {
                    Ok(_) => status_code = 200,
                    Err(SaintMutationError::UniqueConstraintViolationError(..)) => {
                        status_code = 503
                    }
                    Err(SaintMutationError::InvalidSaint) => status_code = 405,
                    _ => status_code = 500,
                }
                saint_response = result.map(Some).unwrap_or_else(|e| {
                    println!("error: {:?}", e);
                    None
                });
            } else {
                saint_response = None;
                status_code = 400;
            }
        }
        method::Method::PUT => {
            saint_collection = None;
            let id = get_id_from_request(&req);
            let value = req.payload().unwrap_or(None);
            if id.is_some() && value.is_some() {
                let lambda_saint_request: Saint = value.unwrap();
                let serialized_saint = serde_json::to_string(&lambda_saint_request).unwrap();
                println!("saint_obj: {}", serialized_saint);
                let result = controller::update_saint(&lambda_saint_request, id.unwrap()).await;
                match result {
                    Ok(_) => status_code = 200,
                    Err(SaintMutationError::UniqueConstraintViolationError(..)) => {
                        status_code = 503
                    }
                    Err(SaintMutationError::InvalidSaint) => status_code = 405,
                    _ => status_code = 500,
                }
                saint_response = result.map(Some).unwrap_or_else(|e| {
                    println!("error: {:?}", e);
                    None
                });
            } else {
                saint_response = None;
                status_code = 400;
            }
        }
        method::Method::DELETE => {
            saint_collection = None;
            saint_response = None;
            if let Some(id) = get_id_from_request(&req) {
                let result = controller::delete_saint(id).await;
                match result {
                    Ok(_) => status_code = 204,
                    Err(SaintMutationError::SaintNotFound) => status_code = 404,
                    _ => status_code = 500,
                }
                if let Err(e) = result {
                    println!("error: {:?}", e);
                }
            } else {
                status_code = 500;
            }
        }
        _ => {
            saint_collection = None;
            saint_response = None;
            status_code = 404;
        }
    }

    let mut content_type = "application/json";
    if status_code == 204 {
        content_type = "";
        println!("status code is 204, removing application/json in Content-Type header")
    }

    let response: Response<Body> = Response::builder()
        .header(CONTENT_TYPE, content_type)
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(ACCESS_CONTROL_ALLOW_HEADERS, "*")
        .header(ACCESS_CONTROL_ALLOW_METHODS, "*")
        .status(status_code)
        .body(if saint_response.is_none() && saint_collection.is_none() {
            Body::Empty
        } else {
            if is_get_saints {
                serde_json::to_string(&saint_collection)
            } else {
                serde_json::to_string(&saint_response)
            }
            .expect("unable to serialize serde_json::Value")
            .into()
        })
        .expect("unable to build http::Response");
    println!(
        "saint HeaderMap.get(CONTENT_TYPE)  {:?}",
        serde_json::to_string(&response.headers().get(CONTENT_TYPE).is_none())
    );
    println!(
        "saint response {:?}",
        serde_json::to_string(&saint_response)
    );
    Ok(response)
}

fn build_sort_criteria_request(
    field: SaintSortFieldRequest,
    direction: SortDirectionRequest,
) -> SaintSortCriteriaRequest {
    SaintSortCriteriaRequest { field, direction }
}
