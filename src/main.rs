mod routes {
    pub mod index;
    pub mod search;
}

mod search;

use actix_web::{web, App, HttpServer};

#[macro_export]
macro_rules! html_snippet {
    ($file:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/html_snippets/", $file))
    };

    ($file:expr, $($var:ident), *) => {
        format!(crate::html_snippet!($file), $($var = $var), *)
    };
    
    ($file:expr, $($key:ident = $value:expr),*) => {
        format!(crate::html_snippet!($file), $($key = $value), *)
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
			.route("/", web::get().to(crate::routes::index::response))
			.route("/search", web::get().to(crate::routes::search::response))
            .service(actix_files::Files::new("/static", "static").show_files_listing())
	})
	.bind("0.0.0.0:8080")?
	.run()
	.await
}
