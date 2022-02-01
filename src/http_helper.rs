use std::collections::HashMap;

use serde::Serialize;


#[derive(Serialize)]
struct JsonifyRequest<'a> {
    pub method: &'a str,
    pub uri: String,
    pub path: &'a str,
    pub query: Option<&'a str>,
    pub version: String,
    pub headers: HashMap<&'a str, &'a str>,
    pub body: serde_json::Value,
}

macro_rules! request_to_serde_json {
    ($req:tt) => {
        {
            let (parts, body) = $req.into_parts();
            let mut map = HashMap::new();
            for (k, v) in parts.headers.iter() {
                map.insert(k.as_str(), v.to_str().unwrap_or("HEADERSCANERR"));
            }

            let body_value = match hyper::body::to_bytes(body).await {
                Ok(b) => {
                    if let Ok(body_str) = std::str::from_utf8(&b) {
                        match serde_json::from_str(body_str) {
                            Ok(obj) => obj,
                            Err(_) => {
                                // if we failed to treat it as json, try to just treat it as a string:
                                serde_json::Value::String(body_str.to_owned())
                            }
                        }
                    } else {
                        serde_json::Value::Null
                    }
                }
                Err(_) => {
                    serde_json::Value::Null
                }
            };

            let json_req = JsonifyRequest {
                method: parts.method.as_str(),
                uri: format!("{}", parts.uri),
                version: format!("{:?}", parts.version),
                headers: map,
                body: body_value,
                path: parts.uri.path(),
                query: parts.uri.query(),
            };

            serde_json::to_value(json_req).unwrap()
        }
    };
}


pub async fn request_to_serde_json_aws(req: lambda_http::Request) -> serde_json::Value {
    request_to_serde_json!(req)
}

pub async fn request_to_serde_json_self(req: hyper::Request<hyper::Body>) -> serde_json::Value {
    request_to_serde_json!(req)
}
