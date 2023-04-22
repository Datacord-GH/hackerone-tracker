pub fn get_hacker_avatar(username: &str) -> String {
    format!(
        "https://hackerone-api.discord.workers.dev/user-avatars/{}",
        username
    )
}
