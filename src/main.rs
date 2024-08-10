use reqwest;
use std::io::stdin;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    println!("What would you like to say?");
    let mut message = String::new();
    let _ = stdin().read_line(&mut message);
    let message_ready = message.trim();

    let channel_id = "put_channel_id_here";

    let auth_token = String::from("put_you_discord_token_here");

    let body = format!("{{\"content\": \"{message_ready}\"}}");

    println!("{body}");

    let client = reqwest::Client::new();
    let reqwest_builder = client
        .post(format!("https://discord.com/api/v9/channels/{channel_id}/messages"))
        .header("Authorization", auth_token)
        .header("Content-Type", "application/json")

        .body(body);

    let request = reqwest_builder.send().await?;
    let response: serde_json::Value = request.json().await?;

    println!("{}", response);

    Ok(())
}
