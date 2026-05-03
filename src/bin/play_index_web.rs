use std::collections::HashMap;
use std::path;
use std::str;
use std::sync;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json,
    Router,
};
use clap::Parser;
use rusqlite::{Connection, Result, OpenFlags};
use rusqlite::types::ValueRef;
use serde_json::json;


const DEFAULT_LISTEN_PORT: u16 = 8000;


#[derive(Parser)]
struct PlayIndexArgs {
    database: path::PathBuf,

    port: Option<u16>,
}

type SharedState = sync::Arc<sync::Mutex<AppState>>;

struct AppState {
    conn: Connection,
}


enum QueryError {
    Unknown,
    PoisonedMutex,
    MissingQuery,
    SqliteFailure(String),
    Utf8Error(String),
}


#[tokio::main]
async fn main() {
    let args = PlayIndexArgs::parse();

    let connection = Connection::open_with_flags(args.database, OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();

    let appstate = AppState {
        conn: connection,
    };
    let state = sync::Arc::new(sync::Mutex::new(appstate));

    let app = Router::new()
        .route("/sql", get(sql_query))
        .with_state(sync::Arc::clone(&state))
        ;
    let address = format!("127.0.0.1:{}", args.port.unwrap_or(DEFAULT_LISTEN_PORT));
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await;
}


async fn sql_query(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<SharedState>,
) -> Result<Json<serde_json::Value>, QueryError> {
    let query_string = params.get("query").ok_or(QueryError::MissingQuery)?;
    let state = state.lock()?;
    let mut statement = state.conn.prepare(query_string)?;
    let column_names: Vec<String> = statement.column_names()
        .into_iter()
        .map(|n| n.to_string())
        .collect();
    let column_count = column_names.len();
    let mut query_rows = statement.query([])?;
    let mut output_rows = Vec::new();
    while let Some(row) = query_rows.next()? {
        let mut output: Vec<serde_json::Value> = Vec::with_capacity(column_count);
        for i in 0..column_count {
            let value = match row.get_ref(i)? {
                ValueRef::Null => {
                    serde_json::Value::Null
                }
                ValueRef::Integer(int) => {
                    serde_json::Value::from(int)
                }
                ValueRef::Real(real) => {
                    serde_json::Value::from(real)
                }
                ValueRef::Text(bytes) => {
                    serde_json::Value::from(str::from_utf8(bytes)?)
                }
                ValueRef::Blob(_) => {
                    // This shouldn't happened based on the expected data.
                    serde_json::Value::from("BLOB")
                }
            };
            output.push(value);
        }
        output_rows.push(output);
    }
    let response = json!({
        "columns": column_names,
        "rows": output_rows,
    });
    Ok(Json(response))
}


impl IntoResponse for QueryError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            QueryError::Unknown => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error".to_string()),
            QueryError::Utf8Error(s) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal error: {}", s)),
            QueryError::PoisonedMutex => (StatusCode::INTERNAL_SERVER_ERROR, "Poisoned mutex".to_string()),
            QueryError::MissingQuery => (StatusCode::BAD_REQUEST, "Missing query parameter".to_string()),
            QueryError::SqliteFailure(s) => (StatusCode::BAD_REQUEST, format!("Sqlite failure: {}", s)),
        };
        let body = Json(json!({"message": message}));
        (status, body).into_response()
    }
}


impl From<str::Utf8Error> for QueryError {
    fn from(_error: str::Utf8Error) -> Self {
        QueryError::Utf8Error("Invalid UTF-8 in TEXT field".to_string())
    }
}

impl<T> From<sync::PoisonError<T>> for QueryError {
    fn from(_error: sync::PoisonError<T>) -> Self {
        QueryError::PoisonedMutex
    }
}

impl From<rusqlite::Error> for QueryError {
    fn from(error: rusqlite::Error) -> Self {
        println!("rusqlite error: {}", error);
        match error {
            rusqlite::Error::SqliteFailure(_e, o) => {
                QueryError::SqliteFailure(o.unwrap_or("".to_string()))
            }
            _ => {
                QueryError::Unknown
            }
        }
    }
}
