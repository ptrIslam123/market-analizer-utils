mod assets;
mod MOEX;

use MOEX::http::api;
use reqwest;

async fn make_request_info(url: String) -> Result<String, String> {
    let stockPriceHistory = reqwest::get(url).await;
    match stockPriceHistory {
        Ok(response) => {
            let content = response.text().await;
            match content {
                Ok(text) => {
                    Ok(text)
                },
                Err(error) => {
                    Err("Could not response context".to_string())
                }
            }
        },
        Err(error) => {
            Err("HTTP Get request failed".to_string())
        }
    }
}

#[tokio::main]
async fn main() {
    
    let tickerName = String::from("SBER");
    let startDate = String::from("2022-01-01");
    let endDate = String::from("2022-01-31");

    let bondsInfo = make_request_info(api::bonds::url_request_builder::list_of_bonds()).await;

    match bondsInfo {
        Ok(context) => {
            let data = api::parser::bond::BondsInfo::new(&context);
            let securities = &data.securities[0];
            let market_data_yield = &data.market_data_yields[0];
            let market_data = &data.market_data[0];

            println!("{} {}",
                market_data_yield.sec_id.as_ref().unwrap(),
                market_data_yield.price_as_percentage() 
                //securities.prev_price.unwrap() // 923,11 * 100 / 1000; 1000 - номинал, 923.11 - рыночная
                
                //securities.prev_wa_price.unwrap()
                //securities.buyback_price.unwrap()
                //securities.yieldat_prev_wa_price.unwrap()
            );

            
        }
        Err(error) => {

        }       
    }
}