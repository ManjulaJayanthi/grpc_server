use axum::{
    http::Method,
    routing::{get, post},
};
use dotenv::dotenv;
use std::{env, net::SocketAddr};
use tokio::join;
use tonic::transport::Server;
use tower_http::cors::{AllowHeaders, Any, CorsLayer};
use tracing_subscriber::FmtSubscriber;

use crate::{
    blog::blog_route::{get_blog_data, thumbs_down, thumbs_up},
    grpc_blog::grpc_blog_route::{
        blog::blog_run_time_server::BlogRunTimeServer, UpdateTheBlogService,
    },
};

mod blog;
mod grpc_blog;

pub async fn internal_server() {
    let internal_port: String =
        env::var("INTERNAL_PORT").expect("INTERNAL_PORT variable should be set");
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
            Method::HEAD,
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

    let subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    // let cors: CorsLayer = CorsLayer::new()
    //     .allow_methods([
    //         Method::GET,
    //         Method::POST,
    //         Method::PATCH,
    //         Method::DELETE,
    //         Method::OPTIONS,
    //         Method::PUT,
    //         Method::HEAD,
    //     ])
    //     .allow_headers(AllowHeaders::any())
    //     .allow_origin(Any)
    //     .expose_headers(ExposeHeaders::any());

    let blog = UpdateTheBlogService::default();
    let blog = BlogRunTimeServer::new(blog);
    let blog_service = tonic_web::enable(blog);

    let grpc_routes = Server::builder()
        .accept_http1(true)
        .add_service(blog_service)
        .into_router();
    // .layer(cors);

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
