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

    duckduckgo::search(&query);

    HttpResponse::Ok()
        .body(format!("{}", query.query))
}
