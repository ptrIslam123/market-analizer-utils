mod request_builder;
mod response_parser;
mod sqlite_proxy;

pub mod http {
    pub mod api {
        
        pub mod stocks {
            pub use crate::MOEX::request_builder::http::api::stocks::url_request_builder;
        }

        pub mod bonds {
            pub use crate::MOEX::request_builder::http::api::bonds::url_request_builder;
        }

        pub mod parser {
            pub use crate::MOEX::response_parser::http::api::parser::*;
        }
    }
}

pub mod sqlite {

    pub mod bonds {
        pub use crate::MOEX::sqlite_proxy::sqlite::bonds::*;
    }
}

