use aws_or_selfhost::{ServerBuilder, ApiResponse, tokio_main, start_handler};

pub async fn root_handler(event: serde_json::Value) -> ApiResponse {
    ApiResponse {
        status_code: 200,
        json: event,
        ..Default::default()
    }
}

fn main() {
    let app = ServerBuilder::default()
        .get("/", root_handler);
    tokio_main(app.start(start_handler!()));
}
