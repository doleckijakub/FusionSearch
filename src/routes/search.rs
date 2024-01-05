use crate::search::{
    Query, ActixQueryWrapper, http_get_text,
    engines::{
        lite_duckduckgo_com,
        bing_com
    }
};

use actix_web::HttpResponse;
use reqwest::Client;

const QUERY_PARAMETER_NAME: &str = "q";

use tokio::task;

macro_rules! execute_search_engine {
    ($query:expr, $engine:ident) => {{
            use crate::search::{Query, SearchResult};

            async fn search_task(query: Query) -> Vec<SearchResult> {
                let request = $engine::request(Client::new(), &query);
                let response = http_get_text(request).await;
                let response = response.unwrap();
                $engine::search_results(&response)
            }

            task::spawn(search_task($query.clone()))
    }};
}

pub async fn response(mut request: ActixQueryWrapper) -> HttpResponse {
    if !request.contains_key(QUERY_PARAMETER_NAME) {
        return HttpResponse::Found()
            .insert_header(("Location", "/"))
            .finish();
    }

    let query = request.get(QUERY_PARAMETER_NAME).unwrap().clone();

    let search_result_futures = {
        let query = Query::from(&mut request);
        vec![
            execute_search_engine!(&query, lite_duckduckgo_com),
            execute_search_engine!(&query, bing_com),
        ]
    };

    let search_results = futures::future::join_all(search_result_futures)
        .await
        .into_iter()
        .flat_map(|join_result| join_result.unwrap())
        .collect::<Vec<_>>()
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
