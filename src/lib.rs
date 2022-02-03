use std::{future::Future, collections::HashMap, pin::Pin};

use serde::de::DeserializeOwned;

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
impl Default for JsonApiResponse {
    fn default() -> Self {
        Self {
            status_code: 500,
            json: serde_json::Value::Null
        }
    }
}

pub type ServerInitResponse = Result<(), ServerInitError>;
pub type ServerInitError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub type BoxDynFuture<O> = Pin<Box<dyn Future<Output = O> + Send>>;
pub type BoxDynFn<I, O> = Box<dyn Fn(I) -> BoxDynFuture<O> + Sync + Send>;
pub type RouteMapInner = HashMap<String, BoxDynFn<serde_json::Value, JsonApiResponse>>;

#[derive(Default)]
pub struct RouteMap {
    pub get_map: RouteMapInner,
    pub post_map: RouteMapInner,
}

/// creates a `BoxDynFn<I, O>` from any callback
/// that is `Sync` and returns a future that is `Send`.
pub fn create_box_dyn_fn<I, O, Out, F>(
    cb: F
) -> BoxDynFn<I, O>
    where F: 'static + Send + Sync + Fn(I) -> Out,
        Out: 'static + Send + Future<Output = O>,
{
    create_box_dyn_fn_from(cb)
}

/// similar to `create_box_dyn_fn` but the input type I
/// must satisfy `From<IOriginal>` where `IOriginal` is
/// the original type that you expect your callback to be called with
/// and then this convenient wrapper calls `.into()` on your behalf.
pub fn create_box_dyn_fn_from<IOriginal, I, O, Out, F>(
    cb: F
) -> BoxDynFn<IOriginal, O>
    where I: From<IOriginal>,
        Out: 'static + Send + Future<Output = O>,
          F: 'static + Send + Sync + Fn(I) -> Out,
{
    let box_dyn_future_cb = move |x: IOriginal| {
        let xi = x.into();
        let res = cb(xi);
        Box::pin(res) as BoxDynFuture<O>
    };
    let box_dyn_future = Box::new(box_dyn_future_cb) as BoxDynFn<IOriginal, O>;
    box_dyn_future
}

/// similar to `create_box_dyn_fn_from` but instead of specifying
/// a 'from', you can specify a conversion function
pub fn create_box_dyn_fn_convert<IOriginal, I, O, Out, F, F2>(
    cb: F,
    convert: F2,
) -> BoxDynFn<IOriginal, O>
    where Out: 'static + Send + Future<Output = O>,
            F: 'static + Send + Sync + Fn(I) -> Out,
           F2: 'static + Send + Sync + Fn(IOriginal) -> I,
{
    let box_dyn_future_cb = move |x: IOriginal| {
        let xi = convert(x);
        let res = cb(xi);
        Box::pin(res) as BoxDynFuture<O>
    };
    let box_dyn_future = Box::new(box_dyn_future_cb) as BoxDynFn<IOriginal, O>;
    box_dyn_future
}

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

    pub fn get<I, F, Out>(
        mut self,
        route: &str,
        f: F
    ) -> Self
        where I: DeserializeOwned,
            Out: 'static + Send + Future<Output = JsonApiResponse>,
              F: 'static + Send + Sync + Fn(I) -> Out,
    {
        let box_dyn: BoxDynFn<serde_json::Value, JsonApiResponse>;
        box_dyn = create_box_dyn_fn_convert(f, |x| serde_json::from_value(x).unwrap());
        self.route_map.get_map.insert(route.to_owned(), box_dyn);
        self
    }

    pub fn post<I, F, Out>(
        mut self,
        route: &str,
        f: F
    ) -> Self
        where I: DeserializeOwned,
            Out: 'static + Send + Future<Output = JsonApiResponse>,
              F: 'static + Send + Sync + Fn(I) -> Out,
    {
        let box_dyn: BoxDynFn<serde_json::Value, JsonApiResponse>;
        box_dyn = create_box_dyn_fn_convert(f, |x| serde_json::from_value(x).unwrap());
        self.route_map.post_map.insert(route.to_owned(), box_dyn);
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

