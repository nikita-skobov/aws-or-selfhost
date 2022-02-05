use aws_or_selfhost::{ServerBuilder, JsonApiResponse, tokio_main};

pub async fn root_handler(event: serde_json::Value) -> JsonApiResponse {
    JsonApiResponse {
        status_code: 200,
        json: event,
        ..Default::default()
    }
}

fn main() {
    let app = ServerBuilder::default()
        .get("/", root_handler);
    tokio_main(app.start(aws_or_selfhost::aws::aws_init));
}
