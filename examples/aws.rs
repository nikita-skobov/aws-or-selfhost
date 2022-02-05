use aws_or_selfhost::{ServerBuilder, ApiResponse, tokio_main};

pub async fn root_handler(event: serde_json::Value) -> ApiResponse {
    let mut resp = ApiResponse::default();
    resp.header("content-type", "text/html");
    resp.body("<html><body><h1>Hello</h1></body></html>");
    resp
}

fn main() {
    let app = ServerBuilder::default()
        .get("/", root_handler);
    tokio_main(app.start(aws_or_selfhost::aws::aws_init));
}
