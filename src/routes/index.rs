use actix_web::HttpResponse;

pub async fn response() -> HttpResponse {
	HttpResponse::Ok()
		.body("Welcome to FusionSearch!")
}
