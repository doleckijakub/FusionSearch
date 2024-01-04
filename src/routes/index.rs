use actix_web::HttpResponse;
use crate::include_static;

pub async fn response() -> HttpResponse {
    HttpResponse::Ok()
        .insert_header(actix_web::http::header::ContentType(mime::TEXT_HTML))
        .body(include_static!("html/index.html"))
}
