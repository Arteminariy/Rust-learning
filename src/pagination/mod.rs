
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};

pub struct ResponsePagination {
    pub size: i64,
    pub page: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestPagination {
    pub total_count: i64,
    pub total_pages: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct List<T> {
    pub pagination: RequestPagination,
    pub items: Vec<T>,
}
