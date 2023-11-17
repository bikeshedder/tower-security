use axum::{
    debug_handler,
    response::{IntoResponse, Redirect},
    routing::get,
    Form, Router,
};
use maud::{html, DOCTYPE};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_security::session::{
    backends::memory::MemorySessionBackend, tower::SessionLayer, Session,
};

#[derive(Clone, Debug)]
struct SessionData {
    data: String,
}

impl tower_security::session::SessionData for SessionData {
    const COOKIE_NAME: &'static str = "example_session";
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let session_storage = MemorySessionBackend::<SessionData>::default();
    let session_layer = SessionLayer::new(session_storage);

    let app = Router::new()
        .route("/", get(root).post(set_session_data))
        .layer(session_layer)
        .layer(tower_cookies::CookieManagerLayer::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 31337));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[debug_handler]
async fn root(session: Session<SessionData>) -> impl IntoResponse {
    html!(
        (DOCTYPE)
        html {
            head {
                link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@2.0.0-alpha1/css/pico.classless.min.css";
                script src="https://unpkg.com/htmx.org@1.9.8" {}
            }
            body hx-boost="true" {
                header {
                    h1 { "tower-security session example" }
                }
                main {
                    h2 {
                        "Session data"
                    }
                    ul {
                        li {
                            @if let Some(session) = &*session.get() {
                                pre {
                                    strong { "Data: " } (session.data)
                                }
                            } @else {
                                em { "session unavailable" }
                            }
                        }

                    }
                    h2 {
                        "Set session data"
                    }
                    form role="group" method="post" {
                        input name="data" placeholder="data";
                        button type="submit" {
                            "Save"
                        }
                    }
                }
            }
        }
    )
}

#[derive(Deserialize)]
struct DataForm {
    data: String,
}

#[debug_handler]
async fn set_session_data(
    session: Session<SessionData>,
    form: Form<DataForm>,
) -> impl IntoResponse {
    session.set(SessionData {
        data: form.data.clone(),
    });
    Redirect::to("/")
}
