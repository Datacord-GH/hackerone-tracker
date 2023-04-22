use crate::models::HackerOneThanks;
use crate::utils::get_hacker_avatar;
use serenity::prelude::SerenityError;
use serenity::{http::Http, model::channel::Embed, model::webhook::Webhook, utils::Colour};
use std::cmp::Ordering;
use std::env;

pub async fn send_new_user(hacker: &HackerOneThanks) -> Result<(), SerenityError> {
    println!("New hacker: {}", hacker.username);

    let http = Http::new("token");
    let token = env::var("HACKERONE_WEBHOOK_URL").expect("missing 'HACKERONE_WEBHOOK_URL' in .env");
    let webhook = Webhook::from_url(&http, &token).await?;

    let hackerone_embed = Embed::fake(|e| {
        e.colour(Colour::from_rgb(0, 0, 1))
            .description(format!(
                "**{}** `({})` was added with **{}** reputation\n\n**Profile:** {}",
                hacker.username, hacker.user_id, hacker.reputation, hacker.profile_url
            ))
            .image(get_hacker_avatar(&hacker.username))
    });

    webhook
        .execute(&http, true, |w| {
            w.content(format!(
                "<@&{}>",
                env::var("ROLE_ID").expect("missing ROLE_ID in .env"),
            ))
            .username("HackerOne Manager")
            .embeds(vec![hackerone_embed])
        })
        .await?;

    Ok(())
}

pub async fn send_updated_rep(
    new_hacker: &HackerOneThanks,
    old_hacker: &HackerOneThanks,
) -> Result<(), SerenityError> {
    println!(
        "Hacker reputation change: {} went from {} to {}",
        new_hacker.username, old_hacker.reputation, new_hacker.reputation
    );

    let http = Http::new("token");
    let token = env::var("HACKERONE_WEBHOOK_URL").expect("missing 'HACKERONE_WEBHOOK_URL' in .env");
    let webhook = Webhook::from_url(&http, &token).await?;
    let what_way = match new_hacker.reputation.cmp(&old_hacker.reputation) {
        Ordering::Less => String::from("decreased"),
        Ordering::Greater => String::from("increased"),
        Ordering::Equal => String::from("didnt change"),
    };

    let hackerone_embed = Embed::fake(|e| {
        e.colour(Colour::from_rgb(0, 0, 1))
            .description(format!(
                "**{}** `({})` reputation {} from **{}** to **{}** `({})`\n\n**Profile:** {}",
                new_hacker.username,
                new_hacker.user_id,
                what_way,
                old_hacker.reputation,
                new_hacker.reputation,
                new_hacker.reputation - old_hacker.reputation,
                new_hacker.profile_url
            ))
            .image(get_hacker_avatar(&new_hacker.username))
    });

    webhook
        .execute(&http, true, |w| {
            w.content(format!(
                "<@&{}>",
                env::var("ROLE_ID").expect("missing ROLE_ID in .env"),
            ))
            .username("HackerOne Manager")
            .embeds(vec![hackerone_embed])
        })
        .await?;

    Ok(())
}