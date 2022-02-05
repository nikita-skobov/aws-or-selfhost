use aws_or_selfhost::{ServerBuilder, ApiResponse, tokio_main, http_helper::FullRequest};

pub async fn root_handler(event: FullRequest) -> ApiResponse {
    let mut resp = ApiResponse::default();
    resp.header("content-type", "text/html");
    resp.body("<html><body><h1>Hello</h1></body></html>");
    resp
}

fn main() {
    let app = ServerBuilder::default()
        .get("/", root_handler);
    tokio_main(app.start(aws_or_selfhost::self_host::selfhost_init));
}
