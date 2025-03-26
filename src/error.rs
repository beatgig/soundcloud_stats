use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct SoundcloudSimpleError {
    #[pyo3(get)]
    message: String,

}

impl SoundcloudSimpleError {
    pub fn new(message: String) -> Self {
        SoundcloudSimpleError { message }
    }
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct SoundcloudRateLimit {
    #[pyo3(get)]
    group: String,
    #[pyo3(get)]
    max_nr_of_requests: u32,
    #[pyo3(get)]
    time_window: String,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct SoundcloudRateLimitMeta {
    #[pyo3(get)]
    rate_limit: SoundcloudRateLimit,
    #[pyo3(get)]
    remaining_requests: u32,
    #[pyo3(get)]
    reset_time: String,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct SoundcloudRateLimitError {
    #[pyo3(get)]
    errors: Vec<SoundcloudRateLimitMeta>,
}

#[pyclass]
#[derive(Clone)]
pub struct SoundcloudError {
    is_rate_limit: bool,
    rate_limit: Option<SoundcloudRateLimitError>,
    simple_error: Option<SoundcloudSimpleError>,
}


#[pymethods]
impl SoundcloudError {
    // Add getters for Rust code
    #[getter]
    pub fn is_rate_limit(&self) -> bool {
        self.is_rate_limit
    }

    #[getter]
    pub fn rate_limit(&self) -> Option<SoundcloudRateLimitError> {
        self.rate_limit.clone()
    }

    #[getter]
    pub fn simple_error(&self) -> Option<SoundcloudSimpleError> {
        self.simple_error.clone()
    }


    // Factory methods for creating errors
    #[staticmethod]
    pub fn new_simple(message: String) -> Self {
        SoundcloudError {
            is_rate_limit: false,
            rate_limit: None,
            simple_error: Some(SoundcloudSimpleError { message }),
        }
    }

    #[staticmethod]
    pub fn new_rate_limit(error: SoundcloudRateLimitError) -> Self {
        SoundcloudError {
            is_rate_limit: true,
            rate_limit: Some(error),
            simple_error: None,
        }
    }

    #[staticmethod]
    pub fn new_rate_limit_message(message: String) -> Self {
        SoundcloudError {
            is_rate_limit: true,
            rate_limit: None,
            simple_error: Some(SoundcloudSimpleError { message }),
        }
    }

    fn __str__(&self) -> PyResult<String> {
        if let Some(simple_error) = &self.simple_error {
            Ok(simple_error.message.clone())
        } else if self.is_rate_limit {
            Ok("Rate limit exceeded".to_string())
        } else {
            Ok("Unknown error".to_string())
        }
    }
    
    // Add a repr for better debugging
    fn __repr__(&self) -> PyResult<String> {
        if self.is_rate_limit {
            Ok(format!("SoundcloudError(rate_limit=True)"))
        } else if let Some(simple_error) = &self.simple_error {
            Ok(format!("SoundcloudError(message={:?})", simple_error.message))
        } else {
            Ok("SoundcloudError(unknown)".to_string())
        }
    }
}