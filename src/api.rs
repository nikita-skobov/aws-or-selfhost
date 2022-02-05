use std::collections::HashMap;
use hyper::HeaderMap;

pub fn header_hashmap_to_header_map(
    map: HashMap<&'static str, String>
) -> HeaderMap {
    let mut headermap = HeaderMap::new();
    for (k, v) in map {
        headermap.insert(k, v.parse().unwrap());
    }
    headermap
}

pub fn body_as_bytes(
    resp_type: ApiResponseType,
) -> Vec<u8> {
    match resp_type {
        ApiResponseType::Json(jv) => {
            let s = serde_json::to_string(&jv).unwrap();
            s.as_bytes().to_vec()
        }
        ApiResponseType::Bytes(b) => {
            b
        }
        ApiResponseType::String(s) => {
            s.as_bytes().to_vec()
        }
    }
}

pub enum ApiResponseType {
    Json(serde_json::Value),
    Bytes(Vec<u8>),
    String(String),
}

impl Default for ApiResponseType {
    fn default() -> Self {
        ApiResponseType::Json(serde_json::Value::Null)
    }
}

impl From<serde_json::Value> for ApiResponseType {
    fn from(v: serde_json::Value) -> Self {
        ApiResponseType::Json(v)
    }
}

impl From<String> for ApiResponseType {
    fn from(v: String) -> Self {
        ApiResponseType::String(v)
    }
}

impl From<&str> for ApiResponseType {
    fn from(v: &str) -> Self {
        ApiResponseType::String(v.to_owned())
    }
}

impl From<Vec<u8>> for ApiResponseType {
    fn from(v: Vec<u8>) -> Self {
        ApiResponseType::Bytes(v)
    }
}

impl From<&[u8]> for ApiResponseType {
    fn from(v: &[u8]) -> Self {
        ApiResponseType::Bytes(v.to_vec())
    }
}

pub struct ApiResponse {
    pub status_code: u16,
    pub resp: ApiResponseType,
    pub headers: HashMap<&'static str, String>,
}
impl Default for ApiResponse {
    fn default() -> Self {
        Self {
            status_code: 500,
            resp: ApiResponseType::default(),
            headers: HashMap::default(),
        }
    }
}

impl ApiResponse {
    pub fn header<V: AsRef<str>>(&mut self, key: &'static str, value: V) {
        self.headers.insert(key, value.as_ref().to_owned());
    }

    pub fn body<T: Into<ApiResponseType>>(&mut self, body: T) {
        self.resp = body.into();
    }
}
