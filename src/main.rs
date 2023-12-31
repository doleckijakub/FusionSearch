mod routes {
    pub mod index;
    pub mod search;
}

mod search;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
			.route("/", web::get().to(crate::routes::index::response))
			.route("/search", web::get().to(crate::routes::search::response))
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}
