use serde::Deserialize;
use sqlx::FromRow;

#[derive(Debug, Clone, Deserialize, FromRow)]
pub struct HackerOneThanks {
    pub username: String,
    pub user_id: String,
    pub reputation: i64,
    pub profile_url: String,
}
