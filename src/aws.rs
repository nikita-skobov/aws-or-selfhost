use crate::{RouteMap, ServerInitResponse, JsonApiResponse, http_helper};


pub async fn aws_init(
    _: [u8; 4],
    _: u16,
    route_map: RouteMap
) -> ServerInitResponse {
    let routes_owned = &route_map;
    let closure = move |event: lambda_http::Request| async move {
        let event_json = http_helper::request_to_serde_json_aws(event).await;
        // TODO: parse out the route from the event json, and match based on
        // the routes owned.
        let request_method = "GET";
        let request_key = "/";

        let route_map_inner = match request_method {
            "GET" => &routes_owned.get_map,
            "POST" => &routes_owned.post_map,
            _ => &routes_owned.get_map
        };
        let json_resp = match route_map_inner.get(request_key) {
            Some(fn_ptr_box) => {
                let future = (fn_ptr_box)(event_json);
                future.await
            }
            None => JsonApiResponse {
                status_code: 500,
                json: serde_json::Value::Null,
            }
        };

        Ok(lambda_http::Response::builder()
            .status(json_resp.status_code)
            .body(serde_json::to_string(&json_resp.json).unwrap())
            .expect("Failed to render response"))
    };

    let func = lambda_http::service_fn(closure);
    lambda_http::run(func).await?;
    Ok(())
}
