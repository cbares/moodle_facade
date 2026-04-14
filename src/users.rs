use crate::request::{Moodlefunctions, Request};
use axum::{Json, extract::Path, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;



/* Extracted from Moodle documentation for core_user_get_users function: 
object {
id int   //ID of the user
username string  Optionnel //The username
firstname string  Optionnel //The first name(s) of the user
lastname string  Optionnel //The family name of the user
fullname string   //The fullname of the user
email string  Optionnel //An email address - allow email as root@localhost
address string  Optionnel //Postal address
phone1 string  Optionnel //Phone 1
phone2 string  Optionnel //Phone 2
department string  Optionnel //department
institution string  Optionnel //institution
idnumber string  Optionnel //An arbitrary ID code number perhaps from the institution
interests string  Optionnel //user interests (separated by commas)
firstaccess int  Optionnel //first access to the site (0 if never)
lastaccess int  Optionnel //last access to the site (0 if never)
auth string  Optionnel //Auth plugins include manual, ldap, etc
suspended int  Optionnel //Suspend user account, either false to enable user login or true to disable it
confirmed int  Optionnel //Active user: 1 if confirmed, 0 otherwise
lang string  Optionnel //Language code such as "en", must exist on server
calendartype string  Optionnel //Calendar type such as "gregorian", must exist on server
theme string  Optionnel //Theme name such as "standard", must exist on server
timezone string  Optionnel //Timezone code such as Australia/Perth, or 99 for default
mailformat int  Optionnel //Mail format code is 0 for plain text, 1 for HTML etc
trackforums int  Optionnel //Whether the user is tracking forums.
description string  Optionnel //User profile description
descriptionformat int  Optionnel //int format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN)
city string  Optionnel //Home city of the user
country string  Optionnel //Home country code of the user, such as AU or CZ
profileimageurlsmall string   //User image profile URL - small version
profileimageurl string   //User image profile URL - big version
customfields  Optionnel //User custom fields (also known as user profile fields)
list of ( 
object {
type string   //The type of the custom field - text field, checkbox...
value string   //The value of the custom field (as stored in the database)
displayvalue string  Optionnel //The value of the custom field for display
name string   //The name of the custom field
shortname string   //The shortname of the custom field - to be able to build the field class in the code
} 
)preferences  Optionnel //Users preferences
list of ( 
object {
name string   //The name of the preferences
value string   //The value of the preference
} 
)} 
)warnings  Optionnel //list of warnings
list of ( 
  //warning
object {
item string  Optionnel //always set to 'key'
itemid int  Optionnel //faulty key name
warningcode string   //the warning code can be used by the client app to implement specific behaviour
message string   //untranslated english message to explain the warning
} 
)
 */
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    id: u64,
    username: Option<String>,
    firstname: Option<String>,
    lastname: Option<String>,
    fullname: String,
    email: Option<String>,
    address: Option<String>,
    phone1: Option<String>,
    phone2: Option<String>,
    department: Option<String>,
    institution: Option<String>,
    idnumber: Option<String>,
    interests: Option<String>,
    firstaccess: Option<u64>,
    lastaccess: Option<u64>,
    auth: Option<String>,
    suspended: Option<bool>,
    confirmed: Option<bool>,
    lang: Option<String>,
    calendartype: Option<String>,
    theme: Option<String>,
    timezone: Option<String>,
    mailformat: Option<u8>,
    trackforums: Option<u8>,
    description: Option<String>,
    descriptionformat: Option<u8>,
    city: Option<String>,
    country: Option<String>,
    profileimageurlsmall: String,
    profileimageurl: String,
    customfields: Option<Vec<CustomField>>,
    preferences: Option<Vec<Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomField {
    r#type: String,
    value: String,
    displayvalue: Option<String>,
    name: String,
    shortname: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Users {
    users: Vec<User>,
    warnings: Vec<Value>,
}

#[derive(Debug, Clone, Copy)]
enum UsersMethod {
    GetUsers,
    GetUserByField,
    #[allow(dead_code)]
    CreateUsers,
    #[allow(dead_code)]
    UpdateUsers,
    #[allow(dead_code)]
    DeleteUsers,
}

impl fmt::Display for UsersMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method_str = match self {
            UsersMethod::GetUsers => "core_user_get_users",
            UsersMethod::GetUserByField => "core_user_get_users_by_field",
            UsersMethod::CreateUsers => "core_user_create_users",
            UsersMethod::UpdateUsers => "core_user_update_users",
            UsersMethod::DeleteUsers => "core_user_delete_users",
        };
        write!(f, "{}", method_str)
    }
}

impl Moodlefunctions for UsersMethod {
    fn get_function_name(&self) -> String {
        self.to_string()
    }
}


/// Handler to fetch users from Moodle and return them as JSON.
pub async fn get_users() -> (StatusCode, Json<Vec<User>>) {
    
    let mut client = Request::new();
    let data_string = [
        ("criteria[0][key]".to_string(),"email".to_string()),
        ("criteria[0][value]".to_string(),"%%".to_string())
        ];
    client.add_query_string(&data_string);
    let response = client.get(UsersMethod::GetUsers).await;
    if response.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]));
    }
    let text = response.unwrap();

    let users_response: Result<Users, serde_json::Error> = serde_json::from_str(&text);
    match users_response {
        Ok(users) => (StatusCode::OK, Json(users.users)),
        Err(err) => {
            eprintln!("Failed to deserialize response from Moodle: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
        }
    }
}

pub async fn get_user_by_id(Path(id): Path<u64>) -> (StatusCode, Json<Vec<User>>) {
    get_user_by_field(Path(("id".to_string(), id.to_string()))).await
}

pub async fn get_user_by_field(Path((field, value)): Path<(String, String)>) -> (StatusCode, Json<Vec<User>>) {
    let mut client = Request::new();
    match field.as_str() {
        "id" | "username" | "email" | "idnumber" => (),
        _ => return (StatusCode::BAD_REQUEST, Json(vec![])),
    }

    let data_string = [
        ("field".to_string(), field),
        ("values[0]".to_string(), value)
        ];
    client.add_query_string(&data_string);

    let response = client.get(UsersMethod::GetUserByField).await;
    if response.is_err() {
        return (response.unwrap_err(), Json(vec![]));
        }

    let text = response.unwrap();
    if text == "[]" {
        return (StatusCode::NOT_FOUND, Json(vec![]));
    }

    match serde_json::from_str::<Vec<User>>(&text){
        Ok(user) => if user.is_empty() {
            (StatusCode::NOT_FOUND, Json(vec![]))
        } else {
            (StatusCode::OK, Json(user))
        },
        Err(err) => {
            eprintln!("Failed to deserialize response from Moodle: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
        }
    }
}