use axum::http::StatusCode;
use axum::response::Html;
use axum::serve;
use axum::{routing::get, Router};
use ssr_rs::Ssr;
use std::cell::RefCell;
use std::fs::read_to_string;
use std::path::Path;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

thread_local! {
    static SSR: RefCell<Ssr<'static, 'static>> = RefCell::new(
        Ssr::from(
            read_to_string(Path::new("./dist/server/server.js").to_str().unwrap()).unwrap(),
            ""
        ).unwrap_or_else(|err| {
            eprintln!("Failed to initialize SSR: {err}");
            std::process::exit(1);
        })
    )
}

async fn index() -> Result<Html<String>, StatusCode> {
    let result = SSR.with(|ssr| {
        let mut ssr = ssr.borrow_mut();
        ssr.render_to_string(None).unwrap_or_else(|err| {
            eprintln!("Error rendering to string: {err}");
            String::new()
        })
    });

    if result.is_empty() {
        eprintln!("Rendered result is empty");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    // NOTE:  For debugging
    // println!("Rendered result: {}", result);

    let result: serde_json::Value = match serde_json::from_str(&result) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Failed to parse JSON: {err}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let head = result["head"].as_str().unwrap_or("");
    let body = result["body"].as_str().unwrap_or("");

    let full_html = format!(
        r#"<!DOCTYPE html>
        <html>
        <head>
            <link rel="stylesheet" href="/client/assets/main.css">
            {head}
        </head>
        <body>
            <div id="svelte-app">{body}</div>
            <script type="module" src="/client/main.js"></script>
        </body>
        </html>"#
    );

    Ok(Html(full_html))
}

#[tokio::main]
async fn main() {
    Ssr::create_platform();
    let app = Router::new()
        // Must use nest_service over route_service here.
        // Not entirely sure why as of writing this comment though.
        .nest_service("/client", ServeDir::new("./dist/client"))
        .route("/", get(index));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    // Copied from the axum static file server example.
    // Not entirely sure what this does as of writing this comment.
    // Intially thought it would help with debugging why the static files weren't being served,
    // but that was fixed by changing `route_service` to `nest_service`.
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::info!("Listening on http://{:?}", listener.local_addr().unwrap());

    serve(listener, app).await.unwrap();
}
