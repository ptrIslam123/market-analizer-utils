mod assets;
mod MOEX;

use MOEX::{http::api::{self, parser::bond::SecuritiesData}, sqlite::bonds::UpdatingFrequency};
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
    // https://docs.rs/sqlite/latest/sqlite/#

    // let tickerName = String::from("SBER");
    // let startDate = String::from("2022-01-01");
    // let endDate = String::from("2022-01-31");

    let bondsInfo = make_request_info(api::bonds::url_request_builder::list_of_bonds()).await;

    match bondsInfo {
        Ok(context) => {
            let data = api::parser::bond::BondsInfo::new(&context);
            let data_securities = &data.securities;
            //let market_data_yield = &data.market_data_yields[0];
            //let market_data = &data.market_data[0];
            

            let mut db_proxy = MOEX::sqlite::bonds::DataBaseProxy::new("db-src/MOEX/bonds.sqlite".to_string());
            db_proxy.open().unwrap();
            match db_proxy.update(1, UpdatingFrequency::Minute, ||{
                for securities in data_securities {
                    match db_proxy.insert_securities_data(&securities) {
                        Ok(()) => {}
                        Err(error) => {
                            println!("{}, {error}", securities.short_name());
                        }
                    }
                }
            }) {
                Ok(result) => {
                    println!("updated={}", result);
                },
                Err(error) => {
                    println!("{}", error);
                }
            }
            

            db_proxy.find_securities_data(|pairs: &[(&str, Option<&str>)]|{
                for &(name, value) in pairs {
                    print!("{}={}\t", name, value.unwrap());
                }
                println!("\n");
            }).unwrap();
        }
        Err(error) => {

        }       
    }
}