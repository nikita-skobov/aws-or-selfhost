use std::{future::Future, collections::HashMap};

pub mod self_host;
pub mod aws;
pub mod http_helper;

pub fn tokio_main(initialization: impl Future<Output = Result<(), ServerInitError>>) {
    let server_init_res = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build().unwrap()
        .block_on(async {
            initialization.await
        });
    if let Err(e) = server_init_res {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

/// Two components to a Json api request:
/// - Json
/// - StatusCode
pub struct JsonApiResponse {
    pub status_code: u16,
    pub json: serde_json::Value,
}

pub type RouteHandler = fn (serde_json::Value) -> JsonApiResponse;
pub type RouteMap = HashMap<String, RouteHandler>;
pub type ServerInitResponse = Result<(), ServerInitError>;
pub type ServerInitError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub struct ServerBuilder {
    pub route_map: RouteMap,
    /// only applicable for self host server
    pub bind_ip: [u8; 4],
    /// only applicable for self host server
    pub listen_port: u16,
}

impl Default for ServerBuilder {
    fn default() -> Self {
        Self {
            route_map: Default::default(),
            bind_ip: [0, 0, 0, 0],
            listen_port: 3000
        }
    }
}

impl ServerBuilder {
    pub fn on_port(mut self, port: u16) -> Self {
        self.listen_port = port;
        self
    }

    pub fn on_ip(mut self, ip: [u8; 4]) -> Self {
        self.bind_ip = ip;
        self
    }

    pub fn get(mut self, route: &str, f: RouteHandler) -> Self {
        self.route_map.insert(route.to_owned(), f);
        self
    }

    pub async fn start<T: Future<Output = Result<(), ServerInitError>>>(
        self,
        start_cb: fn([u8; 4], u16, RouteMap) -> T
    ) -> Result<(), ServerInitError> {
        start_cb(self.bind_ip, self.listen_port, self.route_map).await
    }
}

#[cfg(feature = "aws")]
#[macro_export]
macro_rules! start_handler {
    () => {
        $crate::aws::aws_init
    };
}

#[cfg(not(feature = "aws"))]
#[macro_export]
macro_rules! start_handler {
    () => {
        $crate::self_host::selfhost_init
    };
}

