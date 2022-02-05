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

pub struct FullRequest {
    pub method: String,
    pub uri: String,
    pub path: String,
    pub query: Option<String>,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: serde_json::Value,
}

impl<'a> From<JsonifyRequest<'a>> for FullRequest {
    fn from(v: JsonifyRequest<'a>) -> Self {
        let mut headers = HashMap::new();
        for (key, val) in v.headers.iter() {
            headers.insert(key.to_string(), val.to_string());
        }
        FullRequest {
            method: v.method.into(),
            uri: v.uri.into(),
            path: v.path.into(),
            query: v.query.map(|x| x.to_owned()),
            version: v.version.into(),
            headers,
            body: v.body.into(),
        }
    }
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

            json_req.into()
        }
    };
}


pub async fn request_to_serde_json_aws(
    req: lambda_http::Request
) -> FullRequest {
    request_to_serde_json!(req)
}

pub async fn request_to_serde_json_self(
    req: hyper::Request<hyper::Body>
) -> FullRequest {
    request_to_serde_json!(req)
}
