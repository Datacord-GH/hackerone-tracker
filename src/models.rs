use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct HackerOneThanks {
    pub username: String,
    pub user_id: String,
    pub reputation: i64,
    pub profile_url: String,
}
