use crate::request::{Request, Moodlefunctions};
use axum::{Json, extract::Path, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::fmt;


#[derive(Debug, Deserialize, Serialize)]
pub struct Cohort {
    id: u64,
    name: String,
    idnumber: String,
    description: String,
    descriptionformat: u32,
    visible: bool,
    theme: Option<String>,
    customfields: Vec<CohortCustomField>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CohortCustomField {
    name: String,
    shortname: String,
    r#type: String,
    valueraw: String,
    value: String,
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

/// Handler to fetch cohorts from Moodle and return them as JSON.
pub async fn get_cohorts_by_id(Path(id): Path<u64>) -> (StatusCode, Json<Vec<Cohort>>) {
    let mut client = Request::new();

    let data_string = [
        ("cohortids[0]".to_string(), id.to_string())
        ];
    client.add_query_string(&data_string);
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

