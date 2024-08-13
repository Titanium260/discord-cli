#[derive(serde::Deserialize)]
struct Config {
    token: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let channel_id = String::new();
    let mut menu_selection: String = String::new();

    println!("discord-cli");
    println!("Please specify action...\n");
    println!("0 - show help");
    println!("1 - send message");
    println!("2 - list channels");

    std::io::stdin().read_line(&mut menu_selection).expect("Failed to read line");
    let menu_selection_parsed: i64 = menu_selection.trim().parse().expect("Invalid input");

        //parsing discord token from toml
    let toml: Config = toml::from_str(&std::fs::read_to_string(format!(
        "{}/.discord-cli/config.toml", std::env::var("HOME").unwrap()))
        .unwrap()).unwrap();

    match menu_selection_parsed {
    0 => show_help(),
    1 => send_message(channel_id, toml.token).await?,
    2 => list_channels(),
    _ => do_nothing(),
    }

    Ok(())
}

async fn send_message(channel_id: String, auth_token: String) -> Result<(), reqwest::Error> {

    println!("What would you like to say?");
    let mut message = String::new();
    let _ = std::io::stdin().read_line(&mut message);
    let message_ready = message.trim();

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

async fn fetch_channels() {
    //WIP
}

fn do_nothing() {

}

fn list_channels() {
    println!("WIP, please help me implement this");
}

fn show_help() {
    println!("\nProgram searches for settings in ~/.discord-cli/config.toml");
    println!("Put your Discord token there\n");
}
