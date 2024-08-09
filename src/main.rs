use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let reqwest_builder = reqwest::Client::get(&client, "https://archlinux.org/");
    let request = reqwest::RequestBuilder::send(reqwest_builder).await?;
    let res = reqwest::Response::text(request).await?;

    println!("{}", res);

    Ok(())
}
