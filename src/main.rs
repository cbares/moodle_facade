mod request;
mod cohorts;
mod users;

use axum::{
    Router, routing::get
};

use request::{Request, WebServiceMethod};
// use tracing::{Level, event, instrument};


// #[instrument]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv::dotenv().ok();
    // initialize tracing
    tracing_subscriber::fmt::init();

    let listening_port = std::env::var("LISTENING_PORT").unwrap_or_else(|_| "3000".to_string());
    let listening_ip = std::env::var("LISTENING_IP").unwrap_or_else(|_| "0.0.0.0".to_string());
    let moodle_facade_url = format!("{}:{}", listening_ip, listening_port);
    println!("Starting Moodle Facade on {}", moodle_facade_url);
    //event!(Level::INFO, "Starting Moodle Facade on {}", moodle_facade_url);

    let app = Router::new()
        .route("/", get(root))
        .route("/cohorts", get(cohorts::get_cohorts))
        .route("/cohorts/{id}", get(cohorts::get_cohorts_by_id))
        .route("/users", get(users::get_users))
        .route("/users/{field}/{value}", get(users::get_user_by_field))
        .route("/users/{id}", get(users::get_user_by_id))
        ;

    let listener = tokio::net::TcpListener::bind(moodle_facade_url).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}


/// Handler for the root endpoint to check if the service is running and Moodle is accessible.
async fn root() -> String {
    let client = Request::new();

    let response = client.get(WebServiceMethod::GetSiteInfo).await;
    if response.is_err() {
        return "Moodle_facade is running but failed to connect to Moodle".to_string();
    }
    let text = response.unwrap();
    if text.is_empty() {
        "No data available".to_string()
    } else {
        "Moodle_facade is running and Moodle is accessible".to_string()
    }
}
