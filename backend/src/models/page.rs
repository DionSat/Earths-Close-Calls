use crate::models::neo::Neo;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PagePackageNeo {
    pub neos: Vec<Neo>,
}

impl IntoResponse for PagePackageNeo {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
