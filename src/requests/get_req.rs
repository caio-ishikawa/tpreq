use colored::Colorize;
use reqwest::Response;


pub async fn get_no_header(url: String) -> Result<Response, Box<dyn std::error::Error>> {
    println!("{}", format!("{}", url).blue());
    let res = reqwest::get(url).await?;
    return Ok(res);
}

pub async fn get_with_header(url: String, header_name: String, header_value: String) -> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res: Response = client.get(url).header(header_name, header_value).send().await?;
    return Ok(res);
}