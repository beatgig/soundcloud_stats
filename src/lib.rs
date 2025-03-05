use pyo3::prelude::*;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;
use pyo3::exceptions::PyValueError;

#[derive(Serialize, Deserialize)]
struct AccessTokenResponse {
    access_token: String,
}


#[pyfunction]
fn get_soundcloud_client_id() -> PyResult<String> {

    dotenv().ok();
    match env::var("SOUNDCLOUD_CLIENT_ID") {
        Ok(client_id) => Ok(client_id),
        Err(_) => Err(PyValueError::new_err("SOUNDCLOUD_CLIENT_ID not set")),
    }
}

#[pyfunction]
fn get_soundcloud_client_secret() -> PyResult<String> {
    dotenv().ok();

    match env::var("SOUNDCLOUD_CLIENT_SECRET") {
        Ok(client_secret) => Ok(client_secret),
        Err(_) => Err(PyValueError::new_err("SOUNDCLOUD_CLIENT_SECRET not set")),
    }
}

#[pyfunction]
fn get_soundcloud_access_token(endpoint_url: Option<String>, client_id: Option<String>, client_secret: Option<String>, grant_type: Option<String>) -> PyResult<String> {
    dotenv().ok();

    let client_id = match client_id {
        Some(client_id) => client_id,
        None => get_soundcloud_client_id()?,

    };

    let client_secret = match client_secret {
        Some(client_secret) => client_secret,
        None => get_soundcloud_client_secret()?,
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
            Err(e) => return Err(PyValueError::new_err(format!("Error getting access token: {}", e))),
    };

    if !response.status().is_success() {
        let status = response.status();
        let error_text = match response.text() {
            Ok(text) => text,
            Err(_) => "Could not get error text".to_string(),
        };
        return Err(PyValueError::new_err(format!("API Request failed with status code {}: {}", status, error_text)));
    }

    let token_response: AccessTokenResponse = match response.json() {
        Ok(token_response) => token_response,
        Err(e) => return Err(PyValueError::new_err(format!("Error parsing response: {}", e))),
    };

    Ok(token_response.access_token)

}

/// Python module definition
#[pymodule]
fn soundcloud_stats(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_soundcloud_client_id, m)?)?;
    m.add_function(wrap_pyfunction!(get_soundcloud_client_secret, m)?)?;
    m.add_function(wrap_pyfunction!(get_soundcloud_access_token, m)?)?;
    Ok(())
}
