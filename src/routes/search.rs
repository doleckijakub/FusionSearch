use crate::search::{
    self,
    Query, ActixQueryWrapper, SearchResult, http_get_text,
    engines::{
        duckduckgo_com,
        bing_com,
        google_com,
    }
};

use actix_web::HttpResponse;
use reqwest::Client;

const QUERY_PARAMETER_NAME: &str = "q";

use tokio::task::{self, JoinHandle};

macro_rules! execute_search_engine {
    ($query:expr, $engine:ident) => {{
            use crate::search::{Query, SearchResult};

            async fn search_task(query: Query) -> Result<Vec<SearchResult>, search::Error> {
                let request = $engine::request(Client::new(), &query);
                let response = http_get_text(request).await?;
                Ok($engine::search_results(&response))
            }

            ($engine::URL, task::spawn(search_task($query.clone())))
    }};
}

use std::collections::HashMap;

pub async fn response(mut request: ActixQueryWrapper) -> HttpResponse {
    if !request.contains_key(QUERY_PARAMETER_NAME) {
        return HttpResponse::Found()
            .insert_header(("Location", "/"))
            .finish();
    }

    let query = request.get(QUERY_PARAMETER_NAME).unwrap().clone();

    let (search_summary, search_details, search_results) = {
        let futures: Vec<(
            &'static str,
            JoinHandle<Result<Vec<SearchResult>, search::Error>>
        )> = {
            let query = Query::from(&mut request);
            vec![
                execute_search_engine!(&query, bing_com),
                execute_search_engine!(&query, duckduckgo_com),
                execute_search_engine!(&query, google_com),
            ]
        };

        let maybe_results = futures
            .into_iter()
            .map(|(url, join_handle)| async move {
                (url, join_handle.await)
            });

        let engine_count: u32 = maybe_results.len().try_into().unwrap();
        let mut count_ok: u32 = 0;

        let mut details = String::new();
        let mut scored_results: HashMap<String, (u32, Vec<&'static str>, SearchResult)> = HashMap::new();

        let mut results_total: u32 = 0;

        for maybe_result in maybe_results {
            let (url, res) = maybe_result.await;
            match res.map_err(search::Error::from).and_then(|res| {
                if let Ok(ref r) = res {
                    if r.len() == 0 { return Err(search::Error::NoResultsError); }
                }
                res
            }) {
                Ok(results) => {
                    let num_results = results.len();

                    details.push_str(&crate::include_static!("html/details-success.html",
                        url, num_results
                    ));

                    for result in results {
                        scored_results
                            .entry(result.url.clone())
                            .and_modify(|entry| {
                                let (score, engines, _) = entry;
                                *score += 1;
                                (*engines).push(url);
                                // TODO: change result if is better than current
                            })
                            .or_insert((1, vec![url], result));
                    }

                    count_ok += 1;
                    results_total += num_results as u32;
                },
                Err(err) => {
                    details.push_str(&crate::include_static!("html/details-error.html",
                        url,
                        err
                    ));
                }
            }
        }

        let (results_unique, results) = {
            let mut results_vec = scored_results
                .values()
                .collect::<Vec<_>>();

            results_vec.sort_by(|a, b| {
                b.0.cmp(&a.0)
            });

            (
                results_vec.len(),
                results_vec.into_iter()
                    .map(|entry| crate::include_static!("html/result.html",
                        url = entry.2.url,
                        title = entry.2.title,
                        snippet = entry.2.snippet
                        // TODO: engines = entry.1
                    ))
                    .collect::<String>()
            )
        };


        let summary = crate::include_static!("html/details-summary.html",
            results_total,
            results_unique,
            engine_count,
            count_ok
        );

        (summary, details, results)
    };

    HttpResponse::Ok()
        .body(crate::include_static!("html/search.html",
            query,
            search_summary,
            search_details,
            search_results
        ))
}
