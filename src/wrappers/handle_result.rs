use rocket::serde::json::Json;
use crate::error::{CustomError, ErrorResponse};

pub fn handle_result<T>(result: Result<T, diesel::result::Error>) -> Result<Json<T>, ErrorResponse> {
    result
        .map(Json)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => CustomError::NotFound("Resource not found".into()),
            // diesel::result::Error::DatabaseError(kind, info) => {
            //     match kind {
            //         DatabaseErrorKind::UniqueViolation => {
            //
            //         }
            //         DatabaseErrorKind::ForeignKeyViolation => {
            //
            //         }
            //         DatabaseErrorKind::UnableToSendCommand => {
            //
            //         }
            //         DatabaseErrorKind::SerializationFailure => {
            //
            //         }
            //         DatabaseErrorKind::__Unknown => {
            //             CustomError::InternalServerError(format!("Internal server error: {:?}", e))
            //         }
            //     }
            // }
            _ => CustomError::InternalServerError(format!("Internal server error: {:?}", e)),
        })
        .map_err(ErrorResponse::from)
}
