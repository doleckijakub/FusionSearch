use actix_web::HttpResponse;

pub async fn response() -> HttpResponse {
    HttpResponse::Ok()
        .insert_header(actix_web::http::header::ContentType(mime::TEXT_HTML))
        .body(crate::html_snippet!("index.html"))
}
