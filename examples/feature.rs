use aws_or_selfhost::{ServerBuilder, ApiResponse, tokio_main, start_handler, http_helper::FullRequest};

pub async fn root_handler(event: FullRequest) -> ApiResponse {
    ApiResponse {
        status_code: 200,
        ..Default::default()
    }
}

fn main() {
    let app = ServerBuilder::default()
        .get("/", root_handler);
    tokio_main(app.start(start_handler!()));
}
