use std::net::SocketAddr;

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use iamgroot::{generated::gen, implemented::Impl, jsonrpc::Request};

#[tokio::main]
async fn main() {
    let ctx = Impl {};

    let app = Router::new().route("/api", post(handle)).with_state(ctx);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle(State(rpc): State<Impl>, Json(req): Json<Request>) -> impl IntoResponse {
    let res = gen::handle(&rpc, &req);
    Json(res)
}
