# aws or self host

I would like to write a server application once, and be able to deploy it on AWS, or optionally self host it as a single executable.

This is possible, and can be quite complex, however this library chose to do it in a simple way by purposefully simplifying the problem space. Specifically, we chose to:

- All route handlers receive a json object and must return a json response (status code + json object). This simplifies the differences between aws lambda and a self hosted server such that you can use the same route handler for either one.
- All route handlers are not async (we need this to be able to pass them as function pointers which can be called without the container needing to be mutable (cannot have multiple mutable references to something in a server context)).

# Examples

Simple examples are included in the `examples/` directory.

The `aws` example looks exactly the same as the `self_host` example except that it uses a different initialization callback function. These two examples show how you can use this library by explicitly specifying if you want your server to be ran in aws, or self hosted.

However, you can also decide this at compile time via a macro that uses the aws initialization callback if the `--features aws` cli option is provided at compile time, eg: `cargo build --example feature --features aws`

Whereas the other two examples don't need the feature flag, and instead are explicitly defined, respectively.
