use rocket::response::status::NoContent;
use rocket::serde::json::Json;
use crate::error::{CustomError, ErrorResponse};

pub type ServiceResult<T> = Result<T, CustomError>;

pub type ControllerResult<T> = Result<Json<T>, ErrorResponse>;
pub type NoContentResult = Result<NoContent, ErrorResponse>;

pub fn handle_result<T>(result: ServiceResult<T>) -> ControllerResult<T> {
    result.map(Json).map_err(ErrorResponse::from)
}

pub fn handle_delete_result(result: ServiceResult<()>) -> NoContentResult {
    match result {
        Ok(()) => Ok(NoContent),
        Err(e) => Err(ErrorResponse::from(e)),
    }
}
