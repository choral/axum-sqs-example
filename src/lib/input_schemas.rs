use serde::Deserialize;

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
