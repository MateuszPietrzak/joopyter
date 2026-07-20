# Joopyter

A Jupyter kernels interface.

## Development

Prerequisites:
- [Rust](https://rust-lang.org/learn/get-started/)
- (optionally - for backend hot reloading) [watchexec](https://github.com/watchexec/watchexec)

### Full hot-reloading

For both frontend and backend hot-reloading you need to host the frontend separately from the backend. The frontend will proxy API requests to the server on port `8080`, and use trunk's hot-reloading capabilities. It will be hosted on the port `8000`, where you should be accessing the frontend during development. Port `8080` will work, but not provide you with the hot-reloading.

#### Frontend

```bash
cd frontend
trunk serve --open
```

#### Backend

```bash
cd backend
watchexec -e rs -r cargo run
```

If one does not wish for backend hot-reloading, it can be built simply with `cargo run`. No watch tool is necessary in that case.

### Building a self-contained executable

To build a single binary executable that can serve the app without any additional static assets necessary, first build the frontend.

```bash
cd frontend
trunk build
cd ..
```

Then, building the backend will automatically bundle trunk's output into the executable.

```bash
cd backend
cargo build
```

The resulting binary can be used standalone, containing the entire backend and frontend. The server uses port `8080`.