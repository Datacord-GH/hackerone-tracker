use serde::Deserialize;
use sqlx::FromRow;

#[derive(Debug, Clone, Deserialize, FromRow)]
pub struct HackerOneThanksDB {
    pub username: String,
    pub user_id: String,
    pub reputation: i64,
    pub profile_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HackerOneThanks {
    pub username: String,
    pub id: String,
    pub reputation: i64,
    pub avatar_url: String,
    pub position: usize,
}

impl HackerOneThanks {
    pub fn get_hackerone_url(&self) -> String {
        format!("https://hackerone.com/{}", self.username)
    }

    pub fn get_avatar_url(&self) -> String {
        let default_avatar = String::from("https://hackerone.com/assets/avatars/default-25f7248a18bdf9e2dc8310319b148d66cff430fa0fade6c5f25fee1b8d7f27ed.png");

        if self.avatar_url.len() > 2048 {
            return default_avatar;
        } else if self.avatar_url.starts_with("https") {
            return self.avatar_url.clone();
        } else {
            return default_avatar;
        }
    }
}
