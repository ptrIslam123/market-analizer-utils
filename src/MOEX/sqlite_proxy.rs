use std::fs;
use std::path::Path;
use std::time::SystemTime;
use chrono::{DateTime, Local, NaiveDate, Utc};

pub mod sqlite {

    pub mod bonds {

        use std::{fs::{metadata, remove_file}, alloc::System, time::SystemTime, path::Path};

        use chrono::{DateTime, Utc, Local};
        use reqwest::Error;

        use crate::MOEX::http::api::parser::bond::{SecuritiesData, MarketData, MarketDataYields};

        #[derive(Clone, Copy)]
        pub enum UpdatingFrequency {
            Minute,
            Hour,
            Day,
            Month,
        }

        pub struct DataBaseProxy {
            file_path: String,
            connector: Option<sqlite::Connection>,
            updating_frequency_category: UpdatingFrequency,
            updating_frequency: i64,
        }

        impl DataBaseProxy {
            pub fn new(file_path: String, updating_frequency_category: UpdatingFrequency, updating_frequency: i64) -> DataBaseProxy {
                DataBaseProxy {
                    file_path : file_path,
                    connector: None, 
                    updating_frequency_category : updating_frequency_category, 
                    updating_frequency : updating_frequency
                }
            }

            pub fn open(&mut self) -> Result<(), String> {
                match sqlite::open(&self.file_path) {
                    Ok(connector) => {
                        self.connector = Some(connector);
                        Ok(())
                    },
                    Err(error) => {
                        let error_msg = format!("Could not open sqlite file by path={} because={}",
                            &self.file_path, error.to_string());
                        Err(error_msg)
                    }
                }
            }

            pub fn prepare(&self, data_securities: &Vec<SecuritiesData>, market_data: &Vec<MarketData>, market_data_yields: &Vec<MarketDataYields>) -> Result<(), String> {
                let update_result = match self.update(self.updating_frequency, self.updating_frequency_category, || {
                    // fell securities data table
                    for sc in data_securities {
                        match self.insert_securities_data(&sc) {
                            Ok(()) => {}
                            Err(error) => {
                                println!("Insert securities data failed: {}, {error}", sc.sec_id());
                            }
                        }
                    }
    
                    // fell market data table
                    for md in market_data {
                        match self.insert_market_data(md) {
                            Ok(()) => {},
                            Err(error) => {
                                println!("Insert market data failed: {}, {error}", md.sec_id());
                            }
                        }
                    }
    
                    // fell market data yields table
                    for mdy in market_data_yields {
                        match self.insert_market_data_yields(mdy) {
                            Ok(()) => {},
                            Err(error) => {
                                println!("Insert market data yields failed: {}, {error}", mdy.sec_id());
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
                };
                Ok(())
            }

            pub fn drop(&self) -> Result<(), String> {
                let sql_delete = r#"
                    DROP TABLE SecuritiesData;
                    DROP TABLE MarketData;
                    DROP TABLE MarketDataYields;
                "#.to_string();
                self.execute(&sql_delete)
            }

            pub fn update<F>(&self, frequency: i64, frequency_category: UpdatingFrequency, f: F) -> Result<bool, String> where F: Fn() {
                if self.is_need_update_db(frequency, frequency_category) {
                    self.drop();
                    match self.create_securities_data_table() {
                        Ok(()) => {},
                        Err(error) => {
                            return Err(error);
                        }
                    }

                    match self.create_market_data_table() {
                        Ok(()) => {},
                        Err(error) => {
                            return Err(error);
                        }
                    }

                    match self.create_market_yields_table() {
                        Ok(()) => {
                            f();
                            return Ok(true);
                        },
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }

                Ok(false)
            }

            pub fn find_securities_data<F>(&mut self, f: F) -> Result<(), String> where F: Fn(&[(&str, Option<&str>)]) {
                let sql_select = r#"
                    SELECT * FROM SecuritiesData
                "#;

                match &self.connector {
                    Some(conn) => {
                        conn.iterate(&sql_select, |pairs| {
                            f(pairs);
                            true
                        })
                        .unwrap();
                        Ok(())
                    },
                    None => {
                        Err("Sqlite connector error".to_string())
                    }
                }
            }

            pub fn find_market_data<F>(&mut self, f: F) -> Result<(), String> where F: Fn(&[(&str, Option<&str>)]) {
                let sql_select = r#"
                    SELECT * FROM MarketData
                "#;

                match &self.connector {
                    Some(conn) => {
                        conn.iterate(&sql_select, |pairs| {
                            f(pairs);
                            true
                        })
                        .unwrap();
                        Ok(())
                    },
                    None => {
                        Err("Sqlite connector error".to_string())
                    }
                }
            }

            pub fn find_market_data_yields<F>(&mut self, f: F) -> Result<(), String> where F: Fn(&[(&str, Option<&str>)]) {
                let sql_select = r#"
                    SELECT * FROM MarketDataYields
                "#;

                match &self.connector {
                    Some(conn) => {
                        conn.iterate(&sql_select, |pairs| {
                            f(pairs);
                            true
                        })
                        .unwrap();
                        Ok(())
                    },
                    None => {
                        Err("Sqlite connector error".to_string())
                    }
                }
            }

            pub fn insert_securities_data(&self, securities: &SecuritiesData) -> Result<(), String> {
                let sql_insert = format!("
                    INSERT INTO SecuritiesData (
                        sec_id,
                        short_name,
                        face_value,
                        nominal_value,
                        coupon_value,
                        nominal_coupon_percent,
                        real_coupon_percent,
                        frequency_of_payments_per_year,
                        next_coupon_date,
                        accredint,
                        maturity_date,
                        lot_size,
                        issue_size
                    ) VALUES ('{}', '{}','{}', '{}', '{}','{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}')
                ", 
                securities.sec_id(),
                securities.short_name(),
                securities.face_value(),
                securities.nominal_value(),
                securities.coupon_value(),
                securities.nominal_coupon_percent(),
                securities.real_coupon_percent(),
                securities.frequency_of_payments_per_year(),
                match securities.next_coupon_date() {
                    Some(data) => data,
                    None => String::new(),
                },
                securities.accredint(),
                match securities.maturity_date() {
                    Some(data) => data,
                    None => String::new()
                },
                securities.lot_size(),
                securities.issue_size()
            );
                self.execute(&sql_insert)
            }

            pub fn insert_market_data(&self, market_data: &MarketData) -> Result<(), String> {
                let sql_insert = format!("
                    INSERT INTO MarketData (
                        sec_id,
                        yield_value,
                        market_price_as_percentage,
                        market_price_to_day_as_percentage,
                        volume_today
                    ) VALUES ('{}', '{}','{}', '{}', '{}')
                    ",  
                    market_data.sec_id(),
                    market_data.yield_value(),
                    match market_data.market_price_as_percentage() {
                        Some(price) => price,
                        None => -1.
                    },
                    match market_data.market_price_to_day_as_percentage() {
                        Some(price) => price,
                        None => -1.,
                    },
                    market_data.volume_today(),
                ).to_string();
                self.execute(&sql_insert)
            }

            pub fn insert_market_data_yields(&self, market_data_yields: &MarketDataYields) -> Result<(), String> {
                let sql_insert = format!("
                    INSERT INTO MarketDataYields (
                        sec_id,
                        price_as_percentage
                    ) VALUES ('{}', '{}')
                    ",
                    market_data_yields.sec_id(),
                    market_data_yields.price_as_percentage()
                ).to_string();
                self.execute(&sql_insert)
            }

            pub fn create_securities_data_table(&self) -> Result<(), String>  {
                let sql_table = r#"
                    CREATE TABLE IF NOT EXISTS SecuritiesData (
                        id                                      INTEGER PRIMARY KEY,
                        sec_id                                  TEXT NOT NULL,
                        short_name                              TEXT NOT NULL,
                        face_value                              FLOAT NOT NULL,
                        nominal_value                           FLOAT NOT NULL,
                        coupon_value                            FLOAT NOT NULL,
                        nominal_coupon_percent                  FLOAT NOT NULL,
                        real_coupon_percent                     FLOAT NOT NULL,
                        frequency_of_payments_per_year          INTEGER NOT NULL,
                        next_coupon_date                        TEXT NOT NULL,
                        accredint                               FLOAT NOT NULL,
                        maturity_date                           TEXT NOT NULL,
                        lot_size                                INTEGER NOT NULL,
                        issue_size                              INTEGER NOT NULL
                    )
                "#.to_string();
                self.execute(&sql_table)
            }

            pub fn create_market_data_table(&self) -> Result<(), String> {
                let sql_table = r#"
                    CREATE TABLE IF NOT EXISTS MarketData (
                        id                                      INTEGER PRIMARY KEY,
                        sec_id                                  TEXT NOT NULL,
                        yield_value                             FLOAT NOT NULL,
                        market_price_as_percentage              FLOAT NOT NULL,
                        market_price_to_day_as_percentage       FLOAT NOT NULL,
                        volume_today                            FLOAT NOT NULL
                    )
                "#.to_string();
                self.execute(&sql_table)
            }

            pub fn create_market_yields_table(&self) -> Result<(), String> {
                let sql_table = r#"
                    CREATE TABLE IF NOT EXISTS MarketDataYields (
                        id                                      INTEGER PRIMARY KEY,
                        sec_id                                  TEXT NOT NULL,
                        price_as_percentage                     FLOAT NOT NULL
                    )
                "#.to_string();
                self.execute(&sql_table)
            }

            fn get_last_db_modification(&self) -> Result<DateTime<Local>, String> {
                match metadata(&self.file_path) {
                    Ok(mdata) => {
                        let last_modified_time = mdata.modified().unwrap();
                        Ok(
                            DateTime::<Utc>::from(last_modified_time).with_timezone(&Local)
                        )
                    },
                    Err(error) => {
                        Err(format!("Could not get the last file accessing because: {}", error))
                    }
                }
            }

            fn is_need_update_db(&self, frequency: i64, frequency_category: UpdatingFrequency) -> bool {
                if !Path::new(&self.file_path).exists() {
                    return false;
                }

                let current_date_time = chrono::Local::now();
                let last_db_accessing = self.get_last_db_modification().unwrap();
                let date_time_diff = current_date_time - last_db_accessing;

                match frequency_category {
                    UpdatingFrequency::Minute => {
                        date_time_diff.num_minutes() > frequency
                    },
                    UpdatingFrequency::Hour => {
                        date_time_diff.num_hours() > frequency
                    },
                    UpdatingFrequency::Day => {
                        date_time_diff.num_days() > frequency
                    },
                    UpdatingFrequency::Month => {
                        date_time_diff.num_days() * 30 > frequency
                    }
                }
            }

            fn execute(&self, sql_request: &String) -> Result<(), String> {
                match &self.connector {
                    Some(conn) => {
                        match conn.execute(sql_request) {
                            Ok(()) => {
                                Ok(())
                            },
                            Err(error) => {
                                let error_msg = format!("Sqlite execute operation failed because: {}", error.to_string());
                                Err(error_msg)
                            }
                        }
                    },
                    None => {
                        Err("Sqlite connector error".to_string())
                    }
                } 
            }
        }

    }

}