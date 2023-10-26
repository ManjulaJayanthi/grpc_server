use axum::{
    http::Method,
    routing::{get, post},
};
use dotenv::dotenv;
use std::{env, net::SocketAddr, path::Path, fs::File};
use tokio::join;
use tower_http::cors::{Any, CorsLayer, AllowHeaders};

use crate::blog::blog_route::{
    blog::blog_run_time_server::BlogRunTimeServer, get_blog_data, thumbs_down, thumbs_up,
    UpdateTheBlogService,
};

mod blog;

pub async fn internal_server() {
    let internal_port: String = env::var("INTERNAL_PORT").expect("INTERNAL_PORT variable should be set");
    let internal_addr: SocketAddr = format!("[::0]:{}", internal_port)
        .parse()
        .expect("Unable to parse internal address");

    let cors: CorsLayer = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
            Method::PUT,
            Method::HEAD
        ])
        .allow_headers(AllowHeaders::any())
        .allow_origin(Any);

    let internal_routes = axum::Router::new()
        .route("/", get(|| async { "Hello world!" }))
        .route("/get_mock/:blog", get(get_blog_data))
        .route("/thumbs_up/:blog", post(thumbs_up))
        .route("/thumbs_down/:blog", post(thumbs_down))
        .layer(cors);

    println!("\n Internal Server running on {:?}", &internal_addr);
    axum::Server::bind(&internal_addr)
        .serve(internal_routes.into_make_service())
        .await
        .unwrap()
}

pub async fn grpc_server() {
    let grpc_port = env::var("GRPC_PORT").expect("GRPC_PORT variable should be set");
    let grpc_addr: SocketAddr = format!("[::0]:{}", grpc_port)
        .parse()
        .expect("Unable to parse grpc address");

    let blog_service = UpdateTheBlogService::default();
    let grpc_routes = tonic::transport::Server::builder()
        .add_service(BlogRunTimeServer::new(blog_service))
        .into_router();

    println!("\n gRPC Server running on {:?}", &grpc_addr);

    axum::Server::bind(&grpc_addr)
        .serve(grpc_routes.into_make_service())
        .await
        .unwrap()
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    join!(internal_server(), grpc_server());
}
