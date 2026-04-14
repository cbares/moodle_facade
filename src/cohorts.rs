use crate::request::{Request, Moodlefunctions};
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::fmt;


#[derive(Debug, Deserialize, Serialize)]
pub struct Cohort {
    description: String,
    descriptionformat: u32,
    id: u64,
    idnumber: String,
    name: String,
    theme: String,
    visible: bool,
}

#[derive(Debug, Clone, Copy)]
enum CohortsMethod {
    GetCohorts,
}

impl fmt::Display for CohortsMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method_str = match self {
            CohortsMethod::GetCohorts => "core_cohort_get_cohorts",
        };
        write!(f, "{}", method_str)
    }
}

impl Moodlefunctions for CohortsMethod {
    fn get_function_name(&self) -> String {
        self.to_string()
    }
}


/// Handler to fetch cohorts from Moodle and return them as JSON.
pub async fn get_cohorts() -> (StatusCode, Json<Vec<Cohort>>) {
    
    let client = Request::new();
    let response = client.get(CohortsMethod::GetCohorts).await;
    if response.is_err() {
        return (response.unwrap_err(), Json(vec![]));
    }

    let text = response.unwrap();
    match serde_json::from_str(&text) {
        Ok(cohorts) => (StatusCode::OK, Json(cohorts)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])),
    }
}
