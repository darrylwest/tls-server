
// chatGPT here: https://chatgpt.com/c/6755f01e-90d4-8007-947d-90815655a23c

use axum::{
    extract::Path,
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use std::{
    net::{SocketAddr, TcpListener},
    path::PathBuf,
};
use tokio::fs;

#[tokio::main]
async fn main() {
    // Create a router with two routes: one for the root and another for serving dynamic files.
    let app = Router::new()
        .route("/", get(|| async { "Hello, axum dynamic file serving!" }))
        .route("/:filename", get(serve_dynamic_file));

    // Load the RustlsConfig using the certificate and key files.
    let config = RustlsConfig::from_pem_file(
        "examples/self-signed-certs/cert.pem",
        "examples/self-signed-certs/key.pem",
    )
    .await
    .unwrap();

    // Define the socket address and listener.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).unwrap();
    println!("listening on https://{}", addr);

    // Start the server with the router.
    axum_server::from_tcp_rustls(listener, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Handler for serving dynamic files.
async fn serve_dynamic_file(Path(filename): Path<String>) -> impl IntoResponse {
    // Construct the path to the file. Replace `examples/static` with your static file directory.
    let file_path = PathBuf::from("examples/static").join(&filename);

    // Attempt to read the file.
    match fs::read(file_path).await {
        Ok(contents) => {
            // Determine the Content-Type based on the file extension (basic example).
            let content_type = match filename.split('.').last() {
                Some("html") => "text/html",
                Some("css") => "text/css",
                Some("js") => "application/javascript",
                Some("mp3") => "audio/mpeg",
                Some("png") => "image/png",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                _ => "application/octet-stream", // Default to binary data.
            };
            ([("Content-Type", content_type)], contents.into_response())
        }
        Err(_) => (
            [("Content-Type", "text/plain")],
            "File not found".into_response(),
        ),
    }
}

