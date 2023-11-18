pub mod http {

    pub mod api {

        pub mod stocks {
            pub mod url_request_builder {
                pub fn for_now(ticker: String) -> String {
                    let url = format!("https://iss.moex.com/iss/engines/stock/markets/shares/boards/tqbr/securities/{ticker}.json");
                    url.to_string()
                }
    
                pub fn for_period(ticker: String, start_trade_date: String, end_trade_date: String, start_page_info_index: u32) -> String {
                    let url = format!("https://iss.moex.com/iss/history/engines/stock/markets/shares/boards/tqbr/securities/\n\
                    {ticker}.json?from=\n\
                    {start_trade_date}&till=\n\
                    {end_trade_date}&start=\n\
                    {start_page_info_index}&iss.json=extended&limit=100");
                    url.to_string()
                }
    
                pub fn for_all_time(ticker: String, start_page_info_index: u32) -> String {
                    let start = start_page_info_index.to_string();
                    let url = format!(
                        "https://iss.moex.com/iss/history/engines/stock/markets/shares/securities/\n\
                        {ticker}/history.json?iss.meta=off&start=\n\
                        {start}&limit=100&history.columns=SECID,TRADEDATE,OPEN,CLOSE,VOLUME",
                    );
                    url.to_string()
                }
            }
        }
        
        pub mod bonds {
            pub mod url_request_builder {
                pub fn list_of_bonds() -> String {
                    "https://iss.moex.com/iss/engines/stock/markets/bonds/securities.json".to_string()
                }

                pub fn bond(ticker: String) -> String {
                    let url = format!("https://iss.moex.com/iss/engines/stock/markets/bonds/boards/tqob/securities/{ticker}.json");
                    url.to_string()
                }
            }
        }
        
    }

}