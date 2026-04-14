use serde::{Deserialize, Serialize};
use reqwest::{Certificate, StatusCode};
use tracing::debug;
use std::{fmt, env,};


static ENDPOINT: &str = "/webservice/rest/server.php";

pub trait Moodlefunctions {
    fn get_function_name(&self) -> String;
}

#[derive(Debug, Clone, Copy)]
pub enum WebServiceMethod {
    GetSiteInfo,
    #[allow(dead_code)]
    GetCohortMembers,
    #[allow(dead_code)]
    AddCohortMembers,
    #[allow(dead_code)]
    CreateGroup,
    #[allow(dead_code)]
    GetCourseGroups,
    #[allow(dead_code)]
    AddGroupMembers,
    #[allow(dead_code)]
    GetCourses,
}


impl fmt::Display for WebServiceMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method_str = match self {
            WebServiceMethod::GetSiteInfo => "core_webservice_get_site_info",
            WebServiceMethod::GetCohortMembers => "core_cohort_get_cohort_members",
            WebServiceMethod::AddCohortMembers => "core_cohort_add_cohort_members",
            WebServiceMethod::CreateGroup => "core_group_create_groups",
            WebServiceMethod::GetCourseGroups => "core_group_get_course_groups",
            WebServiceMethod::AddGroupMembers => "core_group_add_group_members",
            WebServiceMethod::GetCourses => "core_course_get_courses",
        };
        write!(f, "{method_str}")
    }
}

impl Moodlefunctions for WebServiceMethod {
    fn get_function_name(&self) -> String {
        self.to_string()
    }
}

// {"exception":"core\\exception\\invalid_parameter_exception",
// "errorcode":"invalidparameter",
// "message":"Valeur incorrecte de param\u00e8tre d\u00e9tect\u00e9e",
// "debuginfo":"Missing required key in single structure: criteria"}    

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub enum MoodleError {
    Exception(String),
    ErrorCode(String),
    Message(String),
    DebugInfo(String),
}

pub struct Request {
    url: String,
    token: String,
    client: reqwest::Client,
    query_string: Option<Vec<(String, String)>>,
}


impl Request {
    /// Creates a new Request instance with the necessary certificate configuration.
    pub fn new() -> Self {
        let cert_path = env::var("CA_CERT_FILE").expect("CA_CERT_FILE must be set in .env file");
        let cert_file = std::fs::read(&cert_path)
            .expect(format!("Failed to read certificate file at {cert_path}").as_str());

        let certificate = Certificate::from_pem(cert_file.as_ref()).expect("Failed to create certificate from PEM file");
        let certificates = [certificate,];
        let client = reqwest::Client::builder()
            .tls_certs_only(certificates)
            .build().unwrap();
        
        Self {
            url: env::var("MOODLE_URL").expect("MOODLE_URL must be set in .env file"),
            token: env::var("MOODLE_TOKEN").expect("MOODLE_TOKEN must be set in .env file"),
            client,
            query_string: None,
        }
    }

    pub fn add_query_string(&mut self, query: &[(String, String)]) {
        self.query_string = Some(query.iter().map(|(k, v)| (k.clone(), v.clone())).collect());
    }


    /// Constructs a GET request to the Moodle API with the specified service.
    pub async fn get<T: Moodlefunctions>(&self, service: T) -> Result<String, StatusCode> {
        let url = format!("{url}{endpoint}", url = self.url, endpoint = ENDPOINT);
        let mut req = self.client.get(&url)
        .query(&[
            ("wstoken", &self.token),
            ("wsfunction", &service.get_function_name()),
            ("moodlewsrestformat", &String::from("json")),
        ]);

        if let Some(query) = &self.query_string {
            req = req.query(query);
        }

        debug!("Sent request: {:?}", req);
        let response = req.send().await;
        if response.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        let text = response.unwrap().text().await;
        if text.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        debug!("Received response from Moodle: {}", text.as_ref().unwrap());
        Ok(text.unwrap())
    }
}
