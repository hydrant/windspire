pub mod commands;
pub mod queries;
pub mod approuter;


pub mod common {
    pub mod http_reponse {
        use axum::{
            http::{header, StatusCode},
            response::IntoResponse,
        };

        
        pub fn json_response<T: serde::Serialize>(status: StatusCode, payload: T) -> impl IntoResponse {
            (
                status,
                [(header::CONTENT_TYPE, "application/json")],
                serde_json::to_string(&payload).unwrap(),
            )
        }
    }
}