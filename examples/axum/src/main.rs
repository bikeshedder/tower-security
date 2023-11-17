use axum::{debug_handler, routing::get, Router};
use std::net::SocketAddr;
use tower_security::{
    session::{tower::SessionLayer, Session},
    MemorySessionStorage,
};

#[derive(Clone, Debug)]
struct SessionData {}

impl tower_security::session::SessionData for SessionData {
    const COOKIE_NAME: &'static str = "session";
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let session_storage = MemorySessionStorage::<SessionData>::default();
    let session_layer = SessionLayer::new(session_storage);

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        // session layer
        .layer(session_layer);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 31337));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[debug_handler]
async fn root(session: Session<SessionData>) -> &'static str {
    println!("Session: {:?}", session);
    //println!("Session: {:?}", session);
    "hello"
}
