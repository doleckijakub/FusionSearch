mod routes {
    pub mod index;
    pub mod search;
}

mod search;

use actix_web::{web, App, HttpServer};

#[macro_export]
macro_rules! include_static {
    ($file:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/", $file))
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
			.route("/", web::get().to(crate::routes::index::response))
			.route("/search", web::get().to(crate::routes::search::response))
	})
	.bind("0.0.0.0:8080")?
	.run()
	.await
}
