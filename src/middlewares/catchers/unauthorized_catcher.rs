use rocket::{catch, Request};
use rocket::serde::json::Json;
use crate::error::ErrorResponse;


#[catch(401)]
pub fn unauthorized_catcher(req: &Request<'_>) -> Json<ErrorResponse> {
    let error_response = req.local_cache(|| ErrorResponse { code: 401, message: "asd".to_string() });
    Json(error_response.clone())
}