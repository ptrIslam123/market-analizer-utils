pub mod http {
    
    pub mod api {

        pub mod parser {

            pub mod stock {

                pub mod now {
                    #[derive(serde::Deserialize, Debug, Clone)]
                    pub struct SecuritiesData {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub sec_id: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub board_id: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub short_name: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub prev_price: Option<f64>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub lot_size: Option<i32>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub face_value: Option<f64>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub status: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub board_name: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub decimals: Option<i32>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub sec_name: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub remarks: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub market_code: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub instr_id: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub sector_id: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub min_step: Option<f64>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub prev_wa_price: Option<f64>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub face_unit: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub prev_date: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub issue_size: Option<i64>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub isin: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub lat_name: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub reg_number: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub prev_legal_close_price: Option<f64>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub currency_id: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub sec_type: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub list_level: Option<i32>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub settle_date: Option<String>,
                    }
    
                    impl SecuritiesData {
                        pub fn new(json_data: &String) -> SecuritiesData {
                            let parsed: serde_json::Value = serde_json::from_str(json_data).unwrap();
                            let securities = parsed.get("securities").unwrap();
                            let securities_data = securities.get("data").unwrap();
                            let records: Vec<SecuritiesData> = serde_json::from_str(&securities_data.to_string()).unwrap();
                            records[0].clone()
                        }
                    }
                }
    
                pub mod for_period {
                    #[derive(serde::Deserialize, Debug)]
                    pub struct SecuritiesData {
                        #[serde(skip_serializing_if = "Option::is_none", rename = "ADMITTEDQUOTE")]
                        pub admitted_quote: Option<f32>,
                        #[serde(skip_serializing_if = "Option::is_none", rename = "ADMITTEDVALUE")]
                        pub admitted_value: Option<f32>,
                        #[serde(skip_serializing_if = "Option::is_none", rename = "BOARDID")]
                        pub board_id: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", rename="CLOSE")]
                        pub close: Option<f32>,
                        #[serde(skip_serializing_if = "Option::is_none", rename = "CURRENCYID")]
                        pub currency_id: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", rename = "HIGH")]
                        pub high: Option<f32>,
                        #[serde(skip_serializing_if = "Option::is_none", rename = "LEGALCLOSEPRICE")]
                        pub legal_close_price: Option<f32>,
                        #[serde(skip_serializing_if = "Option::is_none", rename = "LOW")]
                        pub low: Option<f32>,
                        #[serde(skip_serializing_if = "Option::is_none", rename = "MARKETPRICE2")]
                        pub market_price2: Option<f32>,
                        #[serde(skip_serializing_if = "Option::is_none", rename = "MARKETPRICE3")]
                        pub market_price3: Option<f32>,
                        #[serde(skip_serializing_if = "Option::is_none", rename = "MARKETPRICE3TRADESVALUE")]
                        pub market_price3_trades_value: Option<f32>,
                        #[serde(skip_serializing_if = "Option::is_none", rename = "MP2VALTRD")]
                        pub mp2_val_trd: Option<f32>,
                        #[serde(skip_serializing_if = "Option::is_none", rename = "NUMTRADES")]
                        pub num_trades: Option<u64>,
                        #[serde(skip_serializing_if = "Option::is_none", rename = "OPEN")]
                        pub open: Option<f32>,
                        #[serde(skip_serializing_if = "Option::is_none", rename = "SECID")]
                        pub sec_id: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", rename = "SHORTNAME")]
                        pub short_name: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", rename = "TRADEDATE")]
                        pub trade_date: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", rename = "TRADINGSESSION")]
                        pub trading_session: Option<f32>,
                        #[serde(skip_serializing_if = "Option::is_none", rename = "TRENDCLSPR")]
                        pub trend_close_price: Option<f32>,
                        #[serde(skip_serializing_if = "Option::is_none", rename = "VALUE")]
                        pub value: Option<f32>,
                        #[serde(skip_serializing_if = "Option::is_none", rename = "VOLUME")]
                        pub volume: Option<u64>,
                        #[serde(skip_serializing_if = "Option::is_none", rename = "WAPRICE")]
                        pub wa_price: Option<f32>,
                        #[serde(skip_serializing_if = "Option::is_none", rename = "WAVAL")]
                        pub wa_val: Option<f32>,
                    }
    
                    impl SecuritiesData {
                        pub fn new(json_data: &String) -> Vec<SecuritiesData> {
                            let mut history_securities_data = Vec::new();
                            let parsed: serde_json::Value = serde_json::from_str(json_data).unwrap();
                            let parsed = parsed.as_array().unwrap();
                            let parsed = &parsed[1];
                            let parsed = parsed.get("history").unwrap();
                            let parsed = parsed.as_array().unwrap();
                            let records = parsed[1].as_array().unwrap();
                            for record in records {
                                let data: SecuritiesData = serde_json::from_str(&record.to_string()).unwrap();
                                history_securities_data.push(data);
                            }
    
                            history_securities_data
                        }
                    }
                
                }
    
                pub mod for_all_time {
                    #[derive(serde::Deserialize, Debug)]
                    pub struct SecuritiesData {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub symbol: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub date: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub open: Option<f32>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub close: Option<f32>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub volume: Option<u64>,
                    }
    
                    impl SecuritiesData {
                        pub fn new(json_data: &String) -> Vec<SecuritiesData> {
                            let parsed: serde_json::Value = serde_json::from_str(json_data).unwrap();
                            let history = parsed.get("history").unwrap();
                            let securities_data = history.get("data").unwrap();
                            let history_securities_data: Vec<SecuritiesData> = serde_json::from_str(&securities_data.to_string()).unwrap();
                            history_securities_data
                        }
                    }
                }
    

            }

            pub mod bond {
                pub struct BondsInfo {
                    securities: Vec<SecuritiesData>,
                    market_data: Vec<MarketData>,
                    market_data_yields: Vec<MarketDataYields>,
                }

                #[derive(serde::Deserialize, Debug)]
                struct SecuritiesData {
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SECID")]
                    sec_id: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "BOARDID")]
                    board_id: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SHORTNAME")]
                    shortn_ame: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "PREVWAPRICE")]
                    prev_wa_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "YIELDATPREVWAPRICE")]
                    yieldat_prev_wa_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "COUPONVALUE")]
                    coupon_value: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "NEXTCOUPON")]
                    next_coupon: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "ACCRUEDINT")]
                    accruedint: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "PREVPRICE")]
                    prev_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LOTSIZE")]
                    lot_size: Option<i64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "FACEVALUE")]
                    face_value: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "BOARDNAME")]
                    board_name: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "STATUS")]
                    status: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "MATDATE")]
                    mat_date: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "DECIMALS")]
                    decimals: Option<i64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "COUPONPERIOD")]
                    coupon_period: Option<i64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "ISSUESIZE")]
                    issue_size: Option<i64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "PREVLEGALCLOSEPRICE")]
                    prev_legal_close_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "PREVDATE")]
                    prev_date: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SECNAME")]
                    sec_name: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "REMARKS")]
                    remarks: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "MARKETCODE")]
                    market_code: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "INSTRID")]
                    instr_id: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SECTORID")]
                    sector_id: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "MINSTEP")]
                    min_step: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "FACEUNIT")]
                    face_unit: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "BUYBACKPRICE")]
                    buyback_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "BUYBACKDATE")]
                    buyback_date: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "ISIN")]
                    isin: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LATNAME")]
                    lat_name: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "REGNUMBER")]
                    reg_number: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "CURRENCYID")]
                    currency_id: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "ISSUESIZEPLACED")]
                    issue_size_placed: Option<i64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LISTLEVEL")]
                    list_level: Option<i64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SECTYPE")]
                    sec_type: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SECTORID")]
                    couponpercent: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "OFFERDATE")]
                    offerdate: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SETTLEDATE")]
                    settle_date: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LOTVALUE")]
                    lot_value: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "FACEVALUEONSETTLEDATE")]
                    face_value_on_settle_date: Option<f64>,
                }

                #[derive(serde::Deserialize, Debug)]
                struct MarketData {
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SECID")]
                    sec_id: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "BID")]
                    bid: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "BIDDEPTH")]
                    bid_depth: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "OFFER")]
                    offer: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "OFFERDEPTH")]
                    offer_depth: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SPREAD")]
                    spread: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "BIDDEPTHT")]
                    bid_deptht: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "OFFERDEPTHT")]
                    offer_deptht: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "OPEN")]
                    open: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LOW")]
                    low: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "HIGH")]
                    high: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LAST")]
                    last: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LASTCHANGE")]
                    last_change: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LASTCHANGEPRCNT")]
                    last_change_prcnt: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "QTY")]
                    qty: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "VALUE")]
                    value: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "YIELD")]
                    yield_value: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "VALUE_USD")]
                    value_usd: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "WAPRICE")]
                    wa_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LASTCNGTOLASTWAPRICE")]
                    last_cngto_last_wa_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "WAPTOPREVWAPRICEPRCNT")]
                    wa_p_to_prev_wa_price_prcnt: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "WAPTOPREVWAPRICE")]
                    wa_p_to_prev_wa_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "YIELDATWAPRICE")]
                    yield_at_wa_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "YIELDTOPREVYIELD")]
                    yield_to_prev_yield: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "CLOSEYIELD")]
                    close_yield: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "CLOSEPRICE")]
                    close_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "MARKETPRICETODAY")]
                    market_price_to_day: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "MARKETPRICE")]
                    market_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LASTTOPREVPRICE")]
                    last_to_prev_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "NUMTRADES")]
                    num_trades: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "VOLTODAY")]
                    vol_today: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "VALTODAY")]
                    val_today: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "VALTODAY_USD")]
                    val_today_usd: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "BOARDID")]
                    board_id: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "TRADINGSTATUS")]
                    trading_status: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "UPDATETIME")]
                    update_time: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "DURATION")]
                    duration: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "NUMBIDS")]
                    num_bids: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "NUMOFFERS")]
                    num_offers: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "CHANGE")]
                    change: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "TIME")]
                    time: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "HIGHBID")]
                    highb_id: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LOWOFFER")]
                    low_offer: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "PRICEMINUSPREVWAPRICE")]
                    price_minus_prev_wa_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LASTBID")]
                    lastb_id: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LASTOFFER")]
                    last_offer: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LCURRENTPRICE")]
                    l_current_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LCLOSEPRICE")]
                    l_close_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "MARKETPRICE2")]
                    market_price2: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "OPENPERIODPRICE")]
                    open_period_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SEQNUM")]
                    seq_num: Option<i64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SYSTIME")]
                    systime: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "VALTODAY_RUR")]
                    val_today_rur: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "IRICPICLOSE")]
                    iricpi_close: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "BEICLOSE")]
                    bei_close: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "CBRCLOSE")]
                    cbr_close: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "YIELDTOOFFER")]
                    yield_to_offer: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "YIELDLASTCOUPON")]
                    yield_last_coupon: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "TRADINGSESSION")]
                    trading_session: Option<String>,
                }

                #[derive(serde::Deserialize, Debug)]
                struct MarketDataYields {
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SECID")]
                    sec_id: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "PRICE")]
                    board_id: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "PRICE")]
                    price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "YIELDDATE")]
                    yield_date: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "ZCYCMOMENT")]
                    zcyc_moment: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "YIELDDATETYPE")]
                    yield_date_type: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "EFFECTIVEYIELD")]
                    effective_yield: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "DURATION")]
                    duration: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "ZSPREADBP")]
                    zspreadbp: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "GSPREADBP")]
                    gspreadbp: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "WAPRICE")]
                    wa_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "EFFECTIVEYIELDWAPRICE")]
                    effective_yield_wa_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "DURATIONWAPRICE")]
                    duration_wa_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "IR")]
                    ir: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "ICPI")]
                    icpi: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "BEI")]
                    bei: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "CBR")]
                    cbr: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "YIELDTOOFFER")]
                    yield_to_offer: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "YIELDLASTCOUPON")]
                    yield_last_coupon: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "TRADEMOMENT")]
                    trade_moment: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SEQNUM")]
                    seq_num: Option<i64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SYSTIME")]
                    systime: Option<String>,
                }

                impl BondsInfo {
                    pub fn new(json_data: &String) -> BondsInfo {
                        let parsed: serde_json::Value = serde_json::from_str(json_data).unwrap();
                        let securities = parsed.get("securities").unwrap();
                        let data = securities.get("data").unwrap();
                        let securities_data_records: Vec<SecuritiesData> = serde_json::from_str(&data.to_string()).unwrap();
                    
                        let market_data = parsed.get("marketdata").unwrap();
                        let data = market_data.get("data").unwrap();
                        let market_data_records: Vec<MarketData> = serde_json::from_str(&data.to_string()).unwrap();
                    
                        let marketdata_yields = parsed.get("marketdata_yields").unwrap();
                        let data = marketdata_yields.get("data").unwrap();
                        let market_data_yield_records: Vec<MarketDataYields> = serde_json::from_str(&data.to_string()).unwrap();

                        BondsInfo { securities: securities_data_records, market_data: market_data_records, market_data_yields: market_data_yield_records }
                    }
                }

            }

        }

    }

}