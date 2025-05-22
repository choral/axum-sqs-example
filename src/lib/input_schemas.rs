use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GetUserWithId {
    pub user_id: usize,
}

#[derive(Deserialize)]
pub struct Pagination {
    pub page: usize,
    pub per_page: usize,
}

#[derive(Deserialize, Debug)]
pub struct UserDetail {
    pub user_id: usize,
    pub username: String,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionData {
    pub session_id: String,              // Anonymous or pseudo-random identifier
    pub user_agent: Option<String>,      // Browser or app agent info
    pub ip_address: Option<String>,      // Typically anonymized or truncated
    pub device_type: Option<String>,     // e.g., "mobile", "desktop", etc.
    pub os: Option<String>,              // Operating system info
    pub referrer_url: Option<String>,    // The page that referred the user
    pub start_time: DateTime<Utc>,       // Start of session
    pub end_time: Option<DateTime<Utc>>, // End of session (optional if still ongoing)
    pub pages_visited: Vec<String>,      // URLs or route paths visited
    pub events: Vec<String>,             // Generic events like "button_click", "form_submit"
    pub consent_given: bool,             // Indicates if user consented to tracking
}
