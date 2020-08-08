# Actix Web boilerplate
An Actix web boilerplate with autoreload and API endpoint.
## Requirements
- Rust stable
- Cargo
## Feature support
- Auto reload
- Configuration with `.env`
- logger with `env_logger`
- JSON API Endpoint
- Feature modules
## Folder Structure
- `/src/todo` - Example feature module.
    - `/src/todo/init` - module entry point to hook feature to the main app.
    - `/src/todo/api` - Contains methods for handling requests.
    - `/src/todo/models` - Contains app model declaration.
## Configuration
- copy `example.env` to `.env` then you can set configuration.

## Building
Build using cargo:
```
cargo build
```
## Running
Run using cargo:
```
cargo run
```

Run with auto reload:
```
# install tool
cargo install systemfd cargo-watch

# run
systemfd --no-pid -s http::5000 -- cargo watch -x run
```