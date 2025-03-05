use pyo3::prelude::*;

pub mod auth;
pub mod account;

/// Python module definition
#[pymodule]
fn soundcloud_stats(py: Python, m: &PyModule) -> PyResult<()> {

    let auth_module = PyModule::new(py, "auth")?;

    auth_module.add_function(wrap_pyfunction!(auth::get_soundcloud_client_id, auth_module)?)?;
    auth_module.add_function(wrap_pyfunction!(auth::get_soundcloud_client_secret, auth_module)?)?;
    auth_module.add_function(wrap_pyfunction!(auth::get_soundcloud_access_token, auth_module)?)?;
    
    let account_module = PyModule::new(py, "account")?;
    account_module.add_function(wrap_pyfunction!(account::get_account_stats, account_module)?)?;
    
    m.add_submodule(auth_module)?;
    m.add_submodule(account_module)?;

    py.import("sys")?.getattr("modules")?.set_item("soundcloud_stats.auth", auth_module)?;
    py.import("sys")?.getattr("modules")?.set_item("soundcloud_stats.account", account_module)?;
    Ok(())
}
