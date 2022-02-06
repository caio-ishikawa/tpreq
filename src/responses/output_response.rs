use reqwest::Response;
use colored::Colorize;
use serde_json::Value;

pub async fn print_all(res: Response) { 
    if res.status().is_success() {
        println!("\n{}/{}", format!("HTTP/GET").green(), format!("{}", res.status()).green());
    } else if res.status().is_server_error() {
        println!("\nHTTP/GET/{}", format!("{}", res.status()).red());
    } else {
        println!("\nHTTP/GET/{}", format!("{}", res.status()).yellow());
    }

    for (key, value) in res.headers().iter() {
        match key.as_str() {
            "date" => println!("{}: {:?}", format!("{}", key).yellow(), value),
            "content-type" => println!("{}: {:?}", format!("{}", key).yellow(), value),
            "content-length" => println!("{}: {:?}", format!("{}", key).yellow(), value),
            "connection" => println!("{}: {:?}", format!("{}", key).yellow(), value),
            "access-control-allow-origin" => println!("{}: {:?}", format!("{}", key).yellow(), value),
            "server" => println!("{}: {:?}", format!("{}", key).yellow(), value),
            _ => continue
        }
    }

    let res_text = res.text().await.unwrap();
    let res_json: Value = serde_json::from_str(&res_text).unwrap();

    println!("\n{}: {:#?}", format!("Response").yellow(), res_json)
}