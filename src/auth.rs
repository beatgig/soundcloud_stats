use pyo3::prelude::*;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;
use pyo3::exceptions::PyValueError;
use crate::error;

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct AccessTokenResponse {
    #[pyo3(get)]
    access_token: String,
}

#[pyclass]
#[derive(Clone)]
pub struct AccessTokenResponseResults {
    is_success: bool,
    access_token: Option<AccessTokenResponse>,
    error: Option<error::SoundcloudError>,
}

#[pymethods]
impl AccessTokenResponseResults {
    #[staticmethod]
    fn success(access_token: AccessTokenResponse) -> Self {
        AccessTokenResponseResults {
            is_success: true,
            access_token: Some(access_token),
            error: None,
        }
    }

    #[staticmethod]
    fn error(error: error::SoundcloudError) -> Self {
        AccessTokenResponseResults {
            is_success: false,
            access_token: None,
            error: Some(error),
        }
    }

    #[getter]
    fn is_success(&self) -> bool {
        self.is_success
    }

    #[getter]
    fn is_error(&self) -> bool {
        !self.is_success
    }

    #[getter]
    fn access_token(&self) -> Option<Py<AccessTokenResponse>> {
        match &self.access_token {
            Some(access_token) => Python::with_gil(|py| Some(Py::new(py, access_token.clone()).unwrap())),
            None => None,
        }
    }

    #[getter]
    fn get_error(&self) -> Option<Py<error::SoundcloudError>> {
        match &self.error {
            Some(error) => Python::with_gil(|py| Some(Py::new(py, error.clone()).unwrap())),
            None => None,
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        if self.is_success {
            Ok("AccessTokenResponseResults(Success)".to_string())
        } else {
            Ok("AccessTokenResponseResults(Error)".to_string())
        }
    }
}


#[pyfunction]
pub fn get_soundcloud_client_id() -> PyResult<String> {

    dotenv().ok();
    match env::var("SOUNDCLOUD_CLIENT_ID") {
        Ok(client_id) => Ok(client_id),
        Err(_) => Err(PyValueError::new_err("SOUNDCLOUD_CLIENT_ID not set")),
    }
}

#[pyfunction]
pub fn get_soundcloud_client_secret() -> PyResult<String> {
    dotenv().ok();

    match env::var("SOUNDCLOUD_CLIENT_SECRET") {
        Ok(client_secret) => Ok(client_secret),
        Err(_) => Err(PyValueError::new_err("SOUNDCLOUD_CLIENT_SECRET not set")),
    }
}

#[pyfunction]
pub fn get_soundcloud_access_token(endpoint_url: Option<String>, client_id: Option<String>, client_secret: Option<String>, grant_type: Option<String>) -> PyResult<Py<AccessTokenResponseResults>> {
    dotenv().ok();

    let client_id = match client_id {
        Some(client_id) => client_id,
        None => match get_soundcloud_client_id() {
            Ok(id) => id,
            Err(e) => {
                let error = error::SoundcloudError::new_simple(
                    format!("Failed to get client ID: {}", e)
                );
                return Python::with_gil(|py| Ok(Py::new(py, AccessTokenResponseResults::error(error)).unwrap()));
            }
        },
    };

    let client_secret = match client_secret {
        Some(client_secret) => client_secret,
        None => match get_soundcloud_client_secret() {
            Ok(secret) => secret,
            Err(e) => {
                let error = error::SoundcloudError::new_simple(
                    format!("Failed to get client secret: {}", e)
                );
                return Python::with_gil(|py| Ok(Py::new(py, AccessTokenResponseResults::error(error)).unwrap()));
            }
        },
    };

    let grant_type = match grant_type {
        Some(grant_type) => grant_type,
        None => "client_credentials".to_string(),
    };

    /*
     *
     * $ curl -X POST "https://secure.soundcloud.com/oauth/token" \
     -H  "accept: application/json; charset=utf-8" \
     -H  "Content-Type: application/x-www-form-urlencoded" \
     -H  "Authorization: Basic Base64(client_id:client_secret)" \
     --data-urlencode "grant_type=client_credentials"

    */

    let endpoint_url = match endpoint_url {
        Some(endpoint_url) => endpoint_url,
        None => "https://secure.soundcloud.com/oauth/token".to_string(),
    };

    let client = Client::new();

    let response_result = client.post(&endpoint_url)
        .basic_auth(&client_id, Some(&client_secret))
        .header("accept", "application/json; charset=utf-8")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&[("grant_type", &grant_type)])
        .send();

    let response = match response_result {
        Ok(response) => response,
        Err(e) => {
            let error = error::SoundcloudError::new_simple(
                format!("Error getting access token: {}", e)
            );
            return Python::with_gil(|py| Ok(Py::new(py, AccessTokenResponseResults::error(error)).unwrap()));
        }
    };


    if !response.status().is_success() {
        let status = response.status();
        let error_text = match response.text() {
            Ok(text) => text,
            Err(_) => "Could not get error text".to_string(),
        };
        
        let error = if status.as_u16() == 429 {
            match serde_json::from_str::<error::SoundcloudRateLimitError>(&error_text) {
                Ok(rate_limit_error) => error::SoundcloudError::new_rate_limit(rate_limit_error),
                Err(_) => error::SoundcloudError::new_rate_limit_message(
                    format!("Rate limit exceeded: {}", error_text)
                ),
            }
        } else {
            error::SoundcloudError::new_simple(
                format!("API Request failed with status code {}: {}", status, error_text)
            )
        };
        
        return Python::with_gil(|py| Ok(Py::new(py, AccessTokenResponseResults::error(error)).unwrap()));
    }

    match response.json::<AccessTokenResponse>() {
        Ok(token_response) => {
            Python::with_gil(|py| Ok(Py::new(py, AccessTokenResponseResults::success(token_response)).unwrap()))
        },
        Err(e) => {
            let error = error::SoundcloudError::new_simple(
                format!("Error parsing response: {}", e)
            );
            Python::with_gil(|py| Ok(Py::new(py, AccessTokenResponseResults::error(error)).unwrap()))
        }
    }

}
