use crate::search::{
    Query, ActixQueryWrapper, http_get_text,
    engines::{
        duckduckgo
    }
};

use actix_web::HttpResponse;
use reqwest::Client;

const QUERY_PARAMETER_NAME: &str = "q";

pub async fn response(mut request: ActixQueryWrapper) -> HttpResponse {
    if !request.contains_key(QUERY_PARAMETER_NAME) {
        return HttpResponse::Found()
            .insert_header(("Location", "/"))
            .finish();
    }

    let query = request.get(QUERY_PARAMETER_NAME).unwrap().clone();

    let search_results = {
        let query = Query::from(&mut request);
        let request = duckduckgo::request(Client::new(), &query);
        if let Ok(response) = http_get_text(request).await {
            duckduckgo::search_results(&response)
        } else {
            vec![]
        }
    };

    let search_results = search_results
        .into_iter()
        .map(|result| {
            format!(crate::include_static!("html/result.html"),
                url = result.url,
                title = result.title,
                snippet = result.snippet
            )
        })
        .collect::<String>();

    HttpResponse::Ok()
        .body(format!(crate::include_static!("html/search.html"),
            query = query,
            search_results = search_results
        ))
}
