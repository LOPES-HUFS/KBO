use std::collections::HashMap;

mod config;

use config::url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    println!("{}", url::NAVER);
    println!("{}", url::GITHUB);
    Ok(())
}