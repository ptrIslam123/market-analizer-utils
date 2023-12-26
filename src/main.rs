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
            let market_data_yields = &data.market_data_yields;
            let market_data = &data.market_data;
            

            let mut db_proxy = MOEX::sqlite::bonds::DataBaseProxy::new("db-src/MOEX/bonds.sqlite".to_string(), UpdatingFrequency::Hour, 1);
            db_proxy.open().unwrap();
            db_proxy.prepare(data_securities, market_data, market_data_yields).unwrap();


            println!("-------------------------------------------- Show db info --------------------------------------------");
            db_proxy.find_securities_data(|pairs: &[(&str, Option<&str>)]|{
                for &(name, value) in pairs {
                    print!("{}={}\t", name, value.unwrap());
                }
                println!("\n");
            }).unwrap();

            db_proxy.find_market_data_yields(|pairs: &[(&str, Option<&str>)]|{
                for &(name, value) in pairs {
                    print!("{}={}\t", name, value.unwrap());
                }
                println!("\n");
            }).unwrap();

            db_proxy.find_market_data(|pairs: &[(&str, Option<&str>)]|{
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