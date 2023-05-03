mod discord;
mod models;
mod utils;

use discord::{send_new_user, send_updated_rep};
use dotenv::dotenv;
use models::HackerOneThanks;
use sqlx;
use sqlx::SqlitePool;
use std::env;
use tokio::main;

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    let mut conn = pool.acquire().await?;
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS hackers 
            (user_id TEXT PRIMARY KEY, username TEXT, reputation INTEGER, profile_url TEXT)"#
    )
    .execute(&mut conn)
    .await?;

    println!("Fetching '/thanks' worker");
    let hackers = reqwest::get("https://hackerone-api.discord.workers.dev/thanks")
        .await?
        .json::<Vec<HackerOneThanks>>()
        .await?;

    println!("Found '{}' hacker(s)", hackers.len());

    for hacker in hackers {
        let hacker_db = sqlx::query!(
            r#"SELECT * FROM hackers WHERE user_id = ?1"#,
            hacker.user_id
        )
        .fetch_one(&pool)
        .await;

        match hacker_db {
            Ok(old) => {
                if old.reputation.unwrap() != hacker.reputation {
                    send_updated_rep(
                        &hacker,
                        &HackerOneThanks {
                            profile_url: old.profile_url.unwrap(),
                            user_id: old.user_id.unwrap(),
                            username: old.username.unwrap(),
                            reputation: old.reputation.unwrap(),
                        },
                    )
                    .await?;
                    sqlx::query!(
                        r#"UPDATE hackers SET reputation = ?1 WHERE user_id = ?2"#,
                        hacker.reputation,
                        hacker.user_id
                    )
                    .execute(&mut conn)
                    .await?;
                }
            }
            Err(_) => {
                send_new_user(&hacker).await?;
                sqlx::query!(
                    r#"INSERT INTO hackers (user_id, username, reputation, profile_url) VALUES (?1, ?2, ?3, ?4)"#,
                    hacker.user_id,
                    hacker.username,
                    hacker.reputation,
                    hacker.profile_url
                ).execute(&mut conn)
                .await?;
            }
        }
    }

    Ok(())
}
