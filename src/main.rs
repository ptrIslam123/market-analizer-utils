mod assets;
mod MOEX;

use MOEX::http::api;
use reqwest;

async fn make_request_info(url: String) {
    let stockPriceHistory = reqwest::get(url).await;
    match stockPriceHistory {
        Ok(response) => {
            let content = response.text().await;
            match content {
                Ok(text) => {
                    let stock_data = api::parser::bond::BondsInfo::new(&text);
                },
                Err(error) => {
                    println!("Could not response context")
                }
            }
        },
        Err(error) => {
            println!("HTTP Get request failed");
        }
    }
}

#[tokio::main]
async fn main() {
    
    let tickerName = String::from("SBER");
    let startDate = String::from("2022-01-01");
    let endDate = String::from("2022-01-31");

    make_request_info(
        api::bonds::url_request_builder::list_of_bonds()
    ).await;
}