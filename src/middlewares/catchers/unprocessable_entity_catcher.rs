use rocket::catch;
use rocket::serde::json::Json;
use crate::error::ErrorResponse;

#[catch(422)]
pub fn unprocessable_entity_catcher() -> Json<ErrorResponse> {
    Json(ErrorResponse { code: 422, message: "Unprocessable entity".to_string() })
}