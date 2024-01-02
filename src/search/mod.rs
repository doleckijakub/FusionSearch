use std::collections::HashMap;
use actix_web::web::Query as ActixQuery;
use log::info;

pub mod engines;

pub type ActixQueryWrapper = ActixQuery<HashMap<String, String>>;

pub struct Query {
    pub query: String
}
    
pub struct SearchResult {
    pub url: String,
    pub title: String,
    pub snippet: String
}

impl From<&mut ActixQueryWrapper> for Query {
    fn from(request: &mut ActixQueryWrapper) -> Query {
        let query = request.remove("q").unwrap();

        if !request.is_empty() {
            info!("Unparsed request parameters: {:?}", request.keys());
        }

        Query {
            query
        }
    }
}
