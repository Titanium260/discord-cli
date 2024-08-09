use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.get("https://archlinux.org");

    Ok(())
}
