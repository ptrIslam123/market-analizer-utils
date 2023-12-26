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
                    pub securities: Vec<SecuritiesData>,
                    pub market_data: Vec<MarketData>,
                    pub market_data_yields: Vec<MarketDataYields>,
                }

                #[derive(serde::Deserialize, Debug)]
                pub struct SecuritiesData {
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SECID")]
                    pub sec_id: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "BOARDID")]
                    pub board_id: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SHORTNAME")]
                    pub short_name: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "PREVWAPRICE")]
                    pub prev_wa_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "YIELDATPREVWAPRICE")]
                    pub yieldat_prev_wa_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "COUPONVALUE")]
                    pub coupon_value: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "NEXTCOUPON")]
                    pub next_coupon: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "ACCRUEDINT")]
                    pub accruedint: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "PREVPRICE")]
                    pub prev_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LOTSIZE")]
                    pub lot_size: Option<i64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "FACEVALUE")]
                    pub face_value: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "BOARDNAME")]
                    pub board_name: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "STATUS")]
                    pub status: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "MATDATE")]
                    pub mat_date: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "DECIMALS")]
                    pub decimals: Option<i64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "COUPONPERIOD")]
                    pub coupon_period: Option<i64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "ISSUESIZE")]
                    pub issue_size: Option<i64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "PREVLEGALCLOSEPRICE")]
                    pub prev_legal_close_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "PREVDATE")]
                    pub prev_date: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SECNAME")]
                    pub sec_name: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "REMARKS")]
                    pub remarks: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "MARKETCODE")]
                    pub market_code: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "INSTRID")]
                    pub instr_id: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SECTORID")]
                    pub sector_id: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "MINSTEP")]
                    pub min_step: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "FACEUNIT")]
                    pub face_unit: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "BUYBACKPRICE")]
                    pub buyback_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "BUYBACKDATE")]
                    pub buyback_date: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "ISIN")]
                    pub isin: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LATNAME")]
                    pub lat_name: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "REGNUMBER")]
                    pub reg_number: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "CURRENCYID")]
                    pub currency_id: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "ISSUESIZEPLACED")]
                    pub issue_size_placed: Option<i64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LISTLEVEL")]
                    pub list_level: Option<i64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SECTYPE")]
                    pub sec_type: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SECTORID")]
                    pub coupon_percent: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "OFFERDATE")]
                    pub offer_date: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SETTLEDATE")]
                    pub settle_date: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LOTVALUE")]
                    pub lot_value: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "FACEVALUEONSETTLEDATE")]
                    pub face_value_on_settle_date: Option<f64>,
                }

                impl SecuritiesData {

                    pub fn sec_id(&self) -> String {
                        self.sec_id.clone().unwrap()
                    }

                    /**
                     * @brief Название облигаций.
                     */
                    pub fn short_name(&self) -> String {
                        let short_name = self.short_name.clone().unwrap();
                        short_name.replace("'", "''")
                    }

                    /**
                     * @brief Номинальная стоимость облигаций. 
                     */
                    pub fn face_value(&self) -> f64 {
                        self.face_value.unwrap()
                    }

                    /**
                     * @brief Номинальная стоимость облигаций. 
                     */
                    pub fn nominal_value(&self) -> f64 {
                        self.face_value()
                    }

                    /**
                     * @brief Величина купона. 
                     */
                    pub fn coupon_value(&self) -> f64 {
                        self.coupon_value.unwrap()
                    }

                    /**
                     * @brief Номинальный купонный процент — это процентная ставка, определяющая сумму купонного дохода по облигации или другому
                     * долговому ценному бумагу.Эта ставка рассчитывается как процент от номинальной стоимости облигации и указывается в её условиях выпуска.
                     * Пример: Если купонная облигация имеет номинал 1000 рублей и купонный процент 5%, то владелец облигации будет получать 50 рублей
                     * купонного дохода за каждый купонный период (5% от 1000 рублей).
                     */
                    pub fn nominal_coupon_percent(&self) -> f64 {
                        ((self.coupon_value() / self.nominal_value()) * 100.)
                    }

                    pub fn real_coupon_percent(&self) -> f64 {
                        ((self.coupon_value() / 923.11) * 100.)
                    }

                    pub fn frequency_of_payments_per_year(&self) -> i64 {
                        match self.coupon_period.unwrap() {
                            0 => 0,
                            cp => (365 / cp),
                        }
                    }

                    /**
                     * @brief Ближайшая дата погашения(выплаты) купона.
                     */
                    pub fn next_coupon_date(&self) -> Option<String> {
                        self.next_coupon.clone()
                    }

                    /**
                     * @brief Величчина накопленного купонного дохода.
                     */
                    pub fn accredint(&self) -> f64 {
                        self.accruedint.unwrap()
                    }

                    /**
                     * @brief Дата погашения облигаций.
                     */
                    pub fn maturity_date(&self) -> Option<String> {
                        self.mat_date.clone()
                    }

                    /**
                     * @brief Размер лота.
                     */
                    pub fn lot_size(&self) -> i64 {
                        self.lot_size.unwrap()
                    }

                    /**
                     * @brief Размер выпуска.
                     */
                    pub fn issue_size(&self) -> i64 {
                        self.issue_size.unwrap()
                    }

                }

                #[derive(serde::Deserialize, Debug)]
                pub struct MarketData {
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SECID")]
                    pub sec_id: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "BID")]
                    pub bid: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "BIDDEPTH")]
                    pub bid_depth: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "OFFER")]
                    pub offer: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "OFFERDEPTH")]
                    pub offer_depth: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SPREAD")]
                    pub spread: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "BIDDEPTHT")]
                    pub bid_deptht: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "OFFERDEPTHT")]
                    pub offer_deptht: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "OPEN")]
                    pub open: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LOW")]
                    pub low: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "HIGH")]
                    pub high: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LAST")]
                    pub last: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LASTCHANGE")]
                    pub last_change: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LASTCHANGEPRCNT")]
                    pub last_change_prcnt: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "QTY")]
                    pub qty: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "VALUE")]
                    pub value: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "YIELD")]
                    pub yield_value: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "VALUE_USD")]
                    pub value_usd: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "WAPRICE")]
                    pub wa_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LASTCNGTOLASTWAPRICE")]
                    pub last_cngto_last_wa_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "WAPTOPREVWAPRICEPRCNT")]
                    pub wa_p_to_prev_wa_price_prcnt: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "WAPTOPREVWAPRICE")]
                    pub wa_p_to_prev_wa_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "YIELDATWAPRICE")]
                    pub yield_at_wa_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "YIELDTOPREVYIELD")]
                    pub yield_to_prev_yield: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "CLOSEYIELD")]
                    pub close_yield: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "CLOSEPRICE")]
                    pub close_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "MARKETPRICETODAY")]
                    pub market_price_to_day: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "MARKETPRICE")]
                    pub market_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LASTTOPREVPRICE")]
                    pub last_to_prev_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "NUMTRADES")]
                    pub num_trades: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "VOLTODAY")]
                    pub vol_today: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "VALTODAY")]
                    pub val_today: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "VALTODAY_USD")]
                    pub val_today_usd: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "BOARDID")]
                    pub board_id: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "TRADINGSTATUS")]
                    pub trading_status: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "UPDATETIME")]
                    pub update_time: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "DURATION")]
                    pub duration: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "NUMBIDS")]
                    pub num_bids: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "NUMOFFERS")]
                    pub num_offers: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "CHANGE")]
                    pub change: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "TIME")]
                    pub time: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "HIGHBID")]
                    pub highb_id: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LOWOFFER")]
                    pub low_offer: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "PRICEMINUSPREVWAPRICE")]
                    pub price_minus_prev_wa_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LASTBID")]
                    pub lastb_id: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LASTOFFER")]
                    pub last_offer: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LCURRENTPRICE")]
                    pub l_current_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "LCLOSEPRICE")]
                    pub l_close_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "MARKETPRICE2")]
                    pub market_price2: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "OPENPERIODPRICE")]
                    pub open_period_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SEQNUM")]
                    pub seq_num: Option<i64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SYSTIME")]
                    pub systime: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "VALTODAY_RUR")]
                    pub val_today_rur: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "IRICPICLOSE")]
                    pub iricpi_close: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "BEICLOSE")]
                    pub bei_close: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "CBRCLOSE")]
                    pub cbr_close: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "YIELDTOOFFER")]
                    pub yield_to_offer: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "YIELDLASTCOUPON")]
                    pub yield_last_coupon: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "TRADINGSESSION")]
                    pub trading_session: Option<String>,
                }

                impl MarketData {
                    
                    pub fn sec_id(&self) -> String {
                        self.sec_id.clone().unwrap()
                    }

                    /**
                     * @brief Доходность облигаций к погашению.
                     * Доходность к погашению (Yield to Maturity, YTM):Это расчет, который учитывает все оставшиеся купонные выплаты и
                     * разницу между рыночной ценой облигации и её номинальной стоимостью, которая будет выплачена в конце срока. 
                     * Доходность к погашению рассчитывается с использованием метода внутренней нормы доходности (IRR) и предполагает, что 
                     * все купонные платежи реинвестируются по той же доходности. Формула доходности к погашению требует решения уравнения:
                     * Цена облигации = ∑ (Купонный платеж / (1 + YTM)^t) + (Номинальная стоимость / (1 + YTM)^n)
                     * где `t` — номер купонного периода, а `n` — общее количество купонных выплат до погашения.
                     */
                    pub fn yield_value(&self) -> f64 {
                        self.yield_value.unwrap()
                    }

                    /**
                     * @brief Рыночная цена %.
                     */
                    pub fn market_price_as_percentage(&self) -> Option<f64> {
                        self.market_price
                    }

                    /**
                     * @brief Рыночная цена на сегодня %.
                     */
                    pub fn market_price_to_day_as_percentage(&self) -> Option<f64> {
                        self.market_price_to_day
                    }

                    /**
                     * @brief Объем на сегодня.
                     */
                    pub fn volume_today(&self) -> f64 {
                        self.vol_today.unwrap()
                    }

                }

                #[derive(serde::Deserialize, Debug)]
                pub struct MarketDataYields {
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SECID")]
                    pub sec_id: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "PRICE")]
                    pub board_id: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "PRICE")]
                    pub price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "YIELDDATE")]
                    pub yield_date: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "ZCYCMOMENT")]
                    pub zcyc_moment: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "YIELDDATETYPE")]
                    pub yield_date_type: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "EFFECTIVEYIELD")]
                    pub effective_yield: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "DURATION")]
                    pub duration: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "ZSPREADBP")]
                    pub zspreadbp: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "GSPREADBP")]
                    pub gspreadbp: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "WAPRICE")]
                    pub wa_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "EFFECTIVEYIELDWAPRICE")]
                    pub effective_yield_wa_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "DURATIONWAPRICE")]
                    pub duration_wa_price: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "IR")]
                    pub ir: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "ICPI")]
                    pub icpi: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "BEI")]
                    pub bei: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "CBR")]
                    pub cbr: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "YIELDTOOFFER")]
                    pub yield_to_offer: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "YIELDLASTCOUPON")]
                    pub yield_last_coupon: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "TRADEMOMENT")]
                    pub trade_moment: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SEQNUM")]
                    pub seq_num: Option<i64>,
                    #[serde(skip_serializing_if = "Option::is_none", rename = "SYSTIME")]
                    pub sys_time: Option<String>,
                }

                impl MarketDataYields {

                    pub fn sec_id(&self) -> String {
                        self.sec_id.clone().unwrap()
                    }

                    /**
                     * @brief Руночная цена облигаций %. 
                     */
                    pub fn price_as_percentage(&self) -> f64 {
                        self.price.unwrap()
                    }

                }

                impl BondsInfo {
                    pub fn new(json_data: &String) -> BondsInfo {
                        /* 
                            {
                                "securities": {
                                    "metadata": {},
                                    "columns": ["SECID", "BOARDID", "SHORTNAME", "PREVWAPRICE", "YIELDATPREVWAPRICE", "COUPONVALUE", "NEXTCOUPON", "ACCRUEDINT",
                                        "PREVPRICE", "LOTSIZE", "FACEVALUE", "BOARDNAME", "STATUS", "MATDATE", "DECIMALS", "COUPONPERIOD", "ISSUESIZE", 
                                        "PREVLEGALCLOSEPRICE", "PREVDATE", "SECNAME", "REMARKS", "MARKETCODE", "INSTRID", "SECTORID", "MINSTEP", "FACEUNIT",
                                        "BUYBACKPRICE", "BUYBACKDATE", "ISIN", "LATNAME", "REGNUMBER", "CURRENCYID", "ISSUESIZEPLACED", "LISTLEVEL", "SECTYPE", 
                                        "COUPONPERCENT", "OFFERDATE", "SETTLEDATE", "LOTVALUE", "FACEVALUEONSETTLEDATE"], 
                                    "data": [...]
                                },

                                "marketdata": {
                                    "metadata": {},
                                    "columns": ["SECID", "BID", "BIDDEPTH", "OFFER", "OFFERDEPTH", "SPREAD", "BIDDEPTHT", "OFFERDEPTHT", "OPEN", "LOW", "HIGH",
                                         "LAST", "LASTCHANGE", "LASTCHANGEPRCNT", "QTY", "VALUE", "YIELD", "VALUE_USD", "WAPRICE", "LASTCNGTOLASTWAPRICE", 
                                         "WAPTOPREVWAPRICEPRCNT", "WAPTOPREVWAPRICE", "YIELDATWAPRICE", "YIELDTOPREVYIELD", "CLOSEYIELD", "CLOSEPRICE", 
                                         "MARKETPRICETODAY", "MARKETPRICE", "LASTTOPREVPRICE", "NUMTRADES", "VOLTODAY", "VALTODAY", "VALTODAY_USD", "BOARDID", 
                                         "TRADINGSTATUS", "UPDATETIME", "DURATION", "NUMBIDS", "NUMOFFERS", "CHANGE", "TIME", "HIGHBID", "LOWOFFER", 
                                         "PRICEMINUSPREVWAPRICE", "LASTBID", "LASTOFFER", "LCURRENTPRICE", "LCLOSEPRICE", "MARKETPRICE2", "OPENPERIODPRICE", 
                                         "SEQNUM", "SYSTIME", "VALTODAY_RUR", "IRICPICLOSE", "BEICLOSE", "CBRCLOSE", "YIELDTOOFFER", "YIELDLASTCOUPON", 
                                         "TRADINGSESSION"], 
                                    "data": [...]
                                },
                                "marketdata_yields": {
                                    "metadata": {},
                                    "columns": ["SECID", "BOARDID", "PRICE", "YIELDDATE", "ZCYCMOMENT", "YIELDDATETYPE", "EFFECTIVEYIELD", "DURATION", 
                                            "ZSPREADBP", "GSPREADBP", "WAPRICE", "EFFECTIVEYIELDWAPRICE", "DURATIONWAPRICE", "IR", "ICPI", "BEI", "CBR", 
                                            "YIELDTOOFFER", "YIELDLASTCOUPON", "TRADEMOMENT", "SEQNUM", "SYSTIME"], 
                                    "data": [...]
                                }
                            }
                        */
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