use aws_or_selfhost::{ServerBuilder, JsonApiResponse, tokio_main, start_handler};

pub fn root_handler(event: serde_json::Value) -> JsonApiResponse {
    JsonApiResponse {
        status_code: 200,
        json: event,
    }
}

fn main() {
    let app = ServerBuilder::default()
        .get("/", root_handler);
    tokio_main(app.start(start_handler!()));
}
