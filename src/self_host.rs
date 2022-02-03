use axum::{Router, routing::{get, post}};
use hyper::{Request, Body, StatusCode};
use serde_json::Value;
use std::{net::SocketAddr, pin::Pin, future::Future, sync::Arc};

use crate::{RouteMap, ServerInitResponse, http_helper, JsonApiResponse};

pub fn app_route_get<F>(
    app: Router,
    route_path: &str,
    cb: Arc<F>,
) -> Router
    where F: 'static + Send + Fn(Value) -> Pin<Box<dyn Future<Output = JsonApiResponse> + Send>> + Sync
{
    app.route(route_path, get(move |r: Request<Body>| async move {
        let req_json = http_helper::request_to_serde_json_self(r).await;
        let resp = cb(req_json);
        let resp = resp.await;
        (StatusCode::from_u16(resp.status_code).unwrap(), serde_json::to_string(&resp.json).unwrap())
    }))
}

pub fn app_route_post<F>(
    app: Router,
    route_path: &str,
    cb: Arc<F>,
) -> Router
    where F: 'static + Send + Fn(Value) -> Pin<Box<dyn Future<Output = JsonApiResponse> + Send>> + Sync
{
    app.route(route_path, post(move |r: Request<Body>| async move {
        let req_json = http_helper::request_to_serde_json_self(r).await;
        let resp = cb(req_json);
        let resp = resp.await;
        (StatusCode::from_u16(resp.status_code).unwrap(), serde_json::to_string(&resp.json).unwrap())
    }))
}

pub async fn selfhost_init(
    ip: [u8; 4],
    port: u16,
    mut route_map: RouteMap
) -> ServerInitResponse {
    let mut app = Router::new();
    for (route_path, route_handler) in route_map.get_map.drain() {
        // need to Arc it unfortunately because
        // axum requires a Clone constraint
        let arc = Arc::new(route_handler);
        app = app_route_get(app, &route_path, arc);
    }
    for (route_path, route_handler) in route_map.post_map.drain() {
        // need to Arc it unfortunately because
        // axum requires a Clone constraint
        let arc = Arc::new(route_handler);
        app = app_route_post(app, &route_path, arc);
    }

    let addr = SocketAddr::from((ip, port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
