mod database;

use database::{Answer, Db, Question};

use axum::{
    error_handling::HandleErrorLayer,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use time::Duration;
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;

use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer};

const COUNTER_KEY: &str = "counter";

#[derive(Default, Deserialize, Serialize)]
struct Counter(usize);

async fn handler(session: Session) -> impl IntoResponse {
    let counter: Counter = session.get(COUNTER_KEY).await.unwrap().unwrap_or_default();
    session.insert(COUNTER_KEY, counter.0 + 1).await.unwrap();
    format!("Current count: {}", counter.0)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logging::start();

    let db = Db::default();

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(60 * 5)));

    // Compose the routes
    let app = Router::new()
        .route("/", get(handler))
        .route("/login", post(login))
        .route("/question", post(question))
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(std::time::Duration::from_secs(9))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .layer(session_layer)
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn question(State(_db): State<Db>, Json(question): Json<Question>) -> impl IntoResponse {
    tracing::info!("question => {:?}", question);

    let answer = Answer {
        answer: "TODO".to_string(),
    };

    (StatusCode::CREATED, Json(answer))
}

async fn login(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
