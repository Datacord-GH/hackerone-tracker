mod discord;
mod models;
mod utils;

use discord::{send_new_user, send_updated_rep};
use dotenv::dotenv;
use models::HackerOneThanks;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::env;
use tokio::main;

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let db_url = &env::var("DATABASE_URL")?;

    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        println!("Creating database {}", db_url);
        match Sqlite::create_database(db_url).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let pool = SqlitePool::connect(&db_url).await?;
    let mut conn = pool.acquire().await?;
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS hackers 
            (user_id TEXT PRIMARY KEY, username TEXT, reputation INTEGER, profile_url TEXT)"#,
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
        let hacker_db =
            sqlx::query_as::<_, HackerOneThanks>("SELECT * FROM hackers WHERE user_id = ?")
                .bind(&hacker.user_id)
                .fetch_one(&pool)
                .await;

        match hacker_db {
            Ok(old) => {
                if old.reputation != hacker.reputation {
                    send_updated_rep(
                        &hacker,
                        &HackerOneThanks {
                            profile_url: old.profile_url,
                            user_id: old.user_id,
                            username: old.username,
                            reputation: old.reputation,
                        },
                    )
                    .await?;

                    sqlx::query("UPDATE hackers SET reputation = ? WHERE user_id = ?")
                        .bind(&hacker.reputation)
                        .bind(&hacker.user_id)
                        .execute(&mut conn)
                        .await?;
                }
            }
            Err(_) => {
                send_new_user(&hacker).await?;

                sqlx::query("INSERT INTO hackers (user_id, username, reputation, profile_url) VALUES (?, ?, ?, ?)")
                    .bind(&hacker.user_id)
                    .bind(&hacker.username)
                    .bind(&hacker.reputation)
                    .bind(&hacker.profile_url)
                    .execute(&mut conn)
                    .await?;
            }
        }
    }

    Ok(())
}
