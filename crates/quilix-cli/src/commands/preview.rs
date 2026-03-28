use anyhow::Result;
use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use std::path::PathBuf;
use tower_http::services::ServeDir;

/// Starts a preview server for the built application.
pub fn preview(port: u16) -> Result<()> {
    tracing::info!("Starting Quilix preview server on port {}", port);
    println!("Starting preview server on http://localhost:{}", port);

    let root_dir = std::env::current_dir()?;
    let dist_dir = root_dir.join("dist");

    if !dist_dir.exists() {
        anyhow::bail!("No dist directory found. Run 'quilix build' first.");
    }

    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async { preview_server(port, dist_dir).await })?;

    Ok(())
}

async fn preview_server(port: u16, dist_dir: PathBuf) -> Result<()> {
    let app = Router::new()
        .route("/", get(index_handler))
        .nest_service("/dist", ServeDir::new(&dist_dir));

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("Preview server running at http://localhost:{}", port);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn index_handler() -> impl IntoResponse {
    Html(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Quilix</title>
</head>
<body>
    <div id="root"></div>
    <script type="module" src="/dist/main.js"></script>
</body>
</html>"#,
    )
}
