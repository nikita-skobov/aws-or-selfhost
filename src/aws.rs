use crate::{RouteMap, ServerInitResponse, ApiResponse, http_helper, body_as_bytes};


pub async fn aws_init(
    _: [u8; 4],
    _: u16,
    route_map: RouteMap
) -> ServerInitResponse {
    let routes_owned = &route_map;
    let closure = move |event: lambda_http::Request| async move {
        let event_json = http_helper::request_to_serde_json_aws(event).await;
        let request_method = match event_json.get("method") {
            Some(v) => match v {
                serde_json::Value::String(v) => v.as_str(),
                _ => panic!("Expected method to be a string"),
            }
            None => panic!("Expected method to exist")
        };
        let request_key = match event_json.get("path") {
            Some(p) => match p {
                serde_json::Value::String(p) => p.as_str(),
                _ => panic!("Expected path to be a string"),
            }
            None => panic!("Expected path to exist")
        };

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
            None => ApiResponse {
                status_code: 500,
                ..Default::default()
            }
        };

        let mut builder = lambda_http::Response::builder().status(
            json_resp.status_code
        );
        for (k, v) in json_resp.headers {
            builder = builder.header(k, v);
        }
        let body_bytes = body_as_bytes(json_resp.resp);
        let response = builder.body(body_bytes.to_vec())
            .expect("Failed to render response");

        Ok(response)
    };

    let func = lambda_http::service_fn(closure);
    lambda_http::run(func).await?;
    Ok(())
}
