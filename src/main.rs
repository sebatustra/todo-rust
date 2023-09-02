mod model;
mod response;
mod handler;
mod route;

use axum::http::{
	header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
	HeaderValue, Method
};
use route::create_router;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {

	let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router().layer(cors);

	println!("🚀 Server started successfully");

	axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
		.serve(app.into_make_service())
		.await
		.unwrap();
}
