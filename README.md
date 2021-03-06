# aws or self host

I would like to write a server application once, and be able to deploy it on AWS, or optionally self host it as a single executable.

# Examples

The below example shows how you would use this library. It's goal is to make the route handler as simple to use, while also allowing you to use either callbacks, or function pointers. And the input type can be anything as long as it can be converted from the `FullRequest`.

```rs
use aws_or_selfhost::{ServerBuilder, ApiResponse, tokio_main, http_helper::FullRequest};

pub struct MyEvent {
    pub body: String,
}

impl From<FullRequest> for MyEvent {
    fn from(orig: FullRequest) -> Self {
        MyEvent {
            body: serde_json::to_string(&orig.body).unwrap(),
        }
    }
}

pub struct OtherEvent {
    pub thing: bool,
}

impl From<FullRequest> for OtherEvent {
    fn from(orig: FullRequest) -> Self {
        OtherEvent {
            thing: orig.headers.contains_key("content-type"),
        }
    }
}

pub async fn root_handler(event: FullRequest) -> ApiResponse {
    ApiResponse {
        status_code: 200,
        ..Default::default()
    }
}

pub async fn event3_handler(event: MyEvent) -> ApiResponse {
    ApiResponse {
        status_code: 404,
        ..Default::default()
    }
}

fn main() {
    let app = ServerBuilder::default()
        .get("/", root_handler)
        .get("/event1", |x: MyEvent| async move {
            ApiResponse::default()
        })
        .post("/event2", |y: OtherEvent| async move {
            ApiResponse::default()
        })
        .get("/event3", event3_handler);

    // This will compile the server as an executable for self hosting:
    tokio_main(app.start(aws_or_selfhost::self_host::selfhost_init));
    // alternatively you can use the following if you want an executable
    // suitable for AWS Lambda + API Gateway:
    // tokio_main(app.start(aws_or_selfhost::aws::aws_init));
}
```

More examples are included in the `examples/` directory.

The `aws` example looks exactly the same as the `self_host` example except that it uses a different initialization callback function. These two examples show how you can use this library by explicitly specifying if you want your server to be ran in aws, or self hosted.

However, you can also decide this at compile time via a macro that uses the aws initialization callback if the `--features aws` cli option is provided at compile time, eg: `cargo build --example feature --features aws`

Whereas the other two examples don't need the feature flag, and instead are explicitly defined, respectively.

# License

Copyright ?? 2022 Nikita Skobov

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the ???Software???), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED ???AS IS???, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
