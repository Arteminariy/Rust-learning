use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
}

impl<'r> Responder<'r, 'static> for ErrorResponse {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        Response::build_from(serde_json::to_string(&self).unwrap().respond_to(req)?)
            .status(Status::new(self.code))
            .header(rocket::http::ContentType::JSON)
            .ok()
    }
}

#[derive(Debug)]
pub enum CustomError {
    NotFound(String),
    InternalServerError(String),
}

impl From<CustomError> for ErrorResponse {
    fn from(err: CustomError) -> Self {
        match err {
            CustomError::NotFound(message) => ErrorResponse {
                code: 404,
                message,
            },
            CustomError::InternalServerError(message) => ErrorResponse {
                code: 500,
                message,
            },
        }
    }
}
