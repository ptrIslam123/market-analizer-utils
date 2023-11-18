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
                    print!("{text}");
                },
                Err(error) => {
                    println!("Could not response context")
                }
            }
        },
        Err(error) => {
            println!("HTTP GEt request failed");
        }
    }
}

#[tokio::main]
async fn main() {
    
    let tickerName = String::from("SBER");
    let startDate = String::from("2022-09-01");
    let endDate = String::from("2023-01-31");

    make_request_info(
        api::stocks::url_request_builder::for_now(tickerName)
    ).await;

    // for i  in 0..3 {
    //     make_request_info(
    //         api::bonds::url_request_builder::for_now(tickerName.clone())
    //     ).await;
    // }
}