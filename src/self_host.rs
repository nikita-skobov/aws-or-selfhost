use axum::{Router, routing::get};
use hyper::{Request, Body, StatusCode};
use std::net::SocketAddr;

use crate::{RouteMap, ServerInitResponse, http_helper};


pub async fn selfhost_init(
    ip: [u8; 4],
    port: u16,
    mut route_map: RouteMap
) -> ServerInitResponse {
    let mut app = Router::new();
    for (route_path, route_handler) in route_map.drain() {
        app = app.route(&route_path, get(move |r: Request<Body>| async move {
            let req_json = http_helper::request_to_serde_json_self(r).await;
            let resp = route_handler(req_json);
            // TODO: this is one of the things that implements "IntoResponse"
            // but it might be better to find a more robust builder that
            // also lets user specify headers for example
            (StatusCode::from_u16(resp.status_code).unwrap(), serde_json::to_string(&resp.json).unwrap())
        }))
    }
    let addr = SocketAddr::from((ip, port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
