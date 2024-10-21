use rocket::serde::json::Json;
use crate::error::{CustomError, ErrorResponse};

pub fn handle_result<T>(result: Result<T, diesel::result::Error>) -> Result<Json<T>, ErrorResponse> {
    result
        .map(Json)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => CustomError::NotFound("Resource not found".into()),
            _ => CustomError::InternalServerError(format!("Internal server error: {:?}", e)),
        })
        .map_err(ErrorResponse::from)
}
