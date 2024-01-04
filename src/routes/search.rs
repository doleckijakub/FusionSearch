use actix_web::HttpResponse;
use crate::search::{
    Query, ActixQueryWrapper,
    engines::{
        duckduckgo
    }
};

pub async fn response(mut request: ActixQueryWrapper) -> HttpResponse {
    if !request.contains_key("q") {
        return HttpResponse::Found()
            .insert_header(("Location", "/"))
            .finish();
    }

    let query = Query::from(&mut request);

    let search_results = duckduckgo::search(&query).await.unwrap_or_default();

    HttpResponse::Ok()
        .body(
            search_results
            .into_iter()
            .map(|result| {
                format!(crate::include_static!("html/snippets/result.html"),
                    url = result.url,
                    title = result.title,
                    snippet = result.snippet
                )
            })
            .collect::<String>()
        )
}
