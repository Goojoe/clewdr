use axum::{Json, extract::State};
use axum_auth::AuthBearer;
use rquest::StatusCode;
use tracing::{error, info, warn};

use crate::{
    VERSION_INFO,
    config::{CLEWDR_CONFIG, CookieStatus},
    services::cookie_manager::CookieStatusInfo,
    state::ClientState,
};

/// API endpoint to submit a new cookie
/// Validates and adds the cookie to the cookie manager
///
/// # Arguments
/// * `s` - Application state containing event sender
/// * `t` - Auth bearer token for admin authentication
/// * `c` - Cookie status to be submitted
///
/// # Returns
/// * `StatusCode` - HTTP status code indicating success or failure
pub async fn api_post_cookie(
    State(s): State<ClientState>,
    AuthBearer(t): AuthBearer,
    Json(mut c): Json<CookieStatus>,
) -> StatusCode {
    if !CLEWDR_CONFIG.load().admin_auth(&t) {
        return StatusCode::UNAUTHORIZED;
    }
    if !c.cookie.validate() {
        warn!("Invalid cookie: {}", c.cookie);
        return StatusCode::BAD_REQUEST;
    }
    c.reset_time = None;
    info!("Cookie accepted: {}", c.cookie);
    match s.event_sender.submit(c).await {
        Ok(_) => {
            info!("Cookie submitted successfully");
            StatusCode::OK
        }
        Err(e) => {
            error!("Failed to submit cookie: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

/// API endpoint to retrieve all cookies and their status
/// Gets information about valid, dispatched, exhausted, and invalid cookies
///
/// # Arguments
/// * `s` - Application state containing event sender
/// * `t` - Auth bearer token for admin authentication
///
/// # Returns
/// * `Result<Json<CookieStatusInfo>, (StatusCode, Json<serde_json::Value>)>` - Cookie status info or error
pub async fn api_get_cookies(
    State(s): State<ClientState>,
    AuthBearer(t): AuthBearer,
) -> Result<Json<CookieStatusInfo>, (StatusCode, Json<serde_json::Value>)> {
    if !CLEWDR_CONFIG.load().admin_auth(&t) {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error": "Unauthorized"
            })),
        ));
    }

    match s.event_sender.get_status().await {
        Ok(status) => Ok(Json(status)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to get cookie status: {}", e)
            })),
        )),
    }
}

/// API endpoint to delete a specific cookie
/// Removes the cookie from all collections in the cookie manager
///
/// # Arguments
/// * `s` - Application state containing event sender
/// * `t` - Auth bearer token for admin authentication
/// * `c` - Cookie status to be deleted
///
/// # Returns
/// * `Result<StatusCode, (StatusCode, Json<serde_json::Value>)>` - Success status or error
pub async fn api_delete_cookie(
    State(s): State<ClientState>,
    AuthBearer(t): AuthBearer,
    Json(c): Json<CookieStatus>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    if !CLEWDR_CONFIG.load().admin_auth(&t) {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error": "Unauthorized"
            })),
        ));
    }

    // Convert string to CookieStatus
    if !c.cookie.validate() {
        warn!("Invalid cookie: {}", c.cookie);
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Invalid cookie"
            })),
        ));
    }

    match s.event_sender.delete_cookie(c.to_owned()).await {
        Ok(_) => {
            info!("Cookie deleted successfully: {}", c.cookie);
            Ok(StatusCode::NO_CONTENT)
        }
        Err(e) => {
            error!("Failed to delete cookie: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to delete cookie: {}", e)
                })),
            ))
        }
    }
}

/// API endpoint to get the application version information
///
/// # Returns
/// * `String` - Version information string
pub async fn api_version() -> String {
    VERSION_INFO.to_string()
}

/// API endpoint to verify authentication
/// Checks if the provided token is valid for admin access
///
/// # Arguments
/// * `t` - Auth bearer token to verify
///
/// # Returns
/// * `StatusCode` - OK if authorized, UNAUTHORIZED otherwise
pub async fn api_auth(AuthBearer(t): AuthBearer) -> StatusCode {
    if !CLEWDR_CONFIG.load().admin_auth(&t) {
        return StatusCode::UNAUTHORIZED;
    }
    info!("Auth token accepted,");
    StatusCode::OK
}
