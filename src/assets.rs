pub mod assets {

    pub struct StockInfo {
        tarde_date: String,
        open_price: f32,
        close_price: f32,
        volume: u32,
    }

    pub struct Stock {
        ticker: String,
        name: String,
        market_price: f32,
        history: Vec<StockInfo>,
    }

}