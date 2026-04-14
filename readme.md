# REST API Façade for Moodle

The REST API Façade for Moodle is a Rust-based application that serves as an intermediary layer between clients and the Official Moodle "REST" API.

It is well known that the Official Moodle "REST" API can be complex and difficult to work with, especially for developers who are not familiar with its intricacies. The REST API Façade for Moodle aims to simplify this interaction by providing a more user-friendly and intuitive interface for developers to access Moodle's functionalities.

The REST API Façade for Moodle is designed to respect the principles of RESTful API design while abstracting away the complexities of the underlying Moodle API. It provides a set of endpoints that mirror common Moodle functionalities, allowing developers to interact with Moodle in a more straightforward manner.

## Features

- Simplified API endpoints that abstract away the complexities of the Official Moodle "REST" API.
- Improved error handling and response formatting for better developer experience.
- Support for common Moodle operations such as user management, course management, and more.
- Configurable settings for connecting to the Moodle instance, including support for custom CA certificates.

## Endpoints

The REST API Façade for Moodle provides a set of endpoints that mirror common Moodle functionalities. For example:

- `GET /` to check the health of the API.
- User-related endpoints :
  - `GET /users` to retrieve a list of users.
  - `POST /users` to create a new user.
  - `GET /users/{id}` to retrieve details of a specific user.
  - `GET /users/{field}/{value}` to retrieve users based on a specific field and value.
- Cohort-related endpoints :
  - `GET /cohorts` to retrieve a list of cohorts.
  - `POST /cohorts` to create a new cohort.
  - `GET /cohorts/{id}` to retrieve details of a specific cohort.

## Configuration

The application can be configured using environment variables. An example `.env` file is provided in the repository, which includes the following settings:

- `LISTENING_PORT`: The port on which the API will listen for incoming requests.
- `LISTENING_IP`: The IP address on which the API will listen for incoming requests.
- `MOODLE_URL`: The URL of the Moodle instance to connect to.
- `MOODLE_TOKEN`: The token used for authenticating with the Moodle API.
- `CA_CERT_FILE`: The path to a custom CA certificate file, if needed for secure connections to the Moodle instance.

## Usage

To run the REST API Façade for Moodle, follow these steps :

1. Clone the repository and navigate to the project directory.
2. Create a `.env` file based on the provided `example.env` and fill in the necessary configuration values.
3. Build and run the application using Cargo:

```bash
cargo run
```

1. The API will be available at the configured IP address and port, and you can start making requests to the endpoints.