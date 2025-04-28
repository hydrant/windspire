pub mod approuter;
pub mod commands;
pub mod queries;

pub mod http_response {
    use axum::{
        http::{StatusCode, header},
        response::{IntoResponse, Response},
    };
    use serde::Serialize;
    use serde_json::{json, to_string};

    /// Safer version that never panics
    fn safe_serialize<T: Serialize>(payload: T) -> String {
        to_string(&payload).unwrap_or_else(|_| {
            "{\"success\":false,\"message\":\"Serialization error\"}".to_string()
        })
    }

    pub fn json_response<T: Serialize>(status: StatusCode, payload: T) -> Response {
        (
            status,
            [(header::CONTENT_TYPE, "application/json")],
            safe_serialize(payload),
        )
            .into_response()
    }

    pub fn ok_json_response<T: Serialize>(payload: T) -> Response {
        json_response(StatusCode::OK, json!({ "success": true, "data": payload }))
    }

    pub fn internal_server_error_json_response(err: sqlx::Error) -> Response {
        json_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": err.to_string() }),
        )
    }

    pub fn row_not_found_error_json_response(message: &str) -> Response {
        json_response(
            StatusCode::NOT_FOUND,
            json!({ "success": false, "message": message }),
        )
    }

    pub fn bad_request_error_json_response(message: String) -> Response {
        if message == INVALID_COUNTRY_CODE_MSG {
            json_response(
                StatusCode::BAD_REQUEST,
                json!({ "success": false, "message": "Invalid country code format" }),
            )
        } else {
            json_response(StatusCode::BAD_REQUEST, json!({ "success": false }))
        }
    }
    const INVALID_COUNTRY_CODE_MSG: &str = "Invalid country code";
}
