use pyo3::prelude::*;
use pyo3::types::{PyBytes};
use serde::{Serialize, Deserialize};
use bincode::{serialize, deserialize};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn nacos_sdk_rust_binding_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<ClientOptions>()?;
    m.add_class::<NacosConfigClient>()?;
    m.add_class::<NacosConfigResponse>()?;
    m.add_class::<NacosNamingClient>()?;
    m.add_class::<NacosServiceInstance>()?;
    Ok(())
}

lazy_static::lazy_static! {
    static ref LOG_GUARD: tracing_appender::non_blocking::WorkerGuard = {
      use std::str::FromStr;
      use tracing_subscriber::filter::LevelFilter;
      let log_level = match std::env::var("NACOS_CLIENT_LOGGER_LEVEL") {
        Ok(level) => LevelFilter::from_str(&level).unwrap_or(LevelFilter::INFO),
        Err(_) => LevelFilter::INFO,
      };

      let home_dir = match std::env::var("HOME") {
        Ok(dir) => dir,
        Err(_) => "/tmp".to_string(),
      };
      let file_appender = tracing_appender::rolling::daily(home_dir + "/logs/nacos", "nacos.log");
      let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

      tracing_subscriber::fmt()
        .with_writer(non_blocking)
        // .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339()) // occur `<unknown time>`
        .with_thread_names(true)
        .with_thread_ids(true)
        .with_max_level(log_level)
        .init();

      guard
    };

}

/// log print to console or file
fn init_logger() -> &'static tracing_appender::non_blocking::WorkerGuard {
    &LOG_GUARD
}

#[pyclass]
#[derive(Clone, Serialize, Deserialize)]
pub struct ClientOptions {
    /// Server Addr, e.g. address:port[,address:port],...]
    #[pyo3(set, get)]
    pub server_addr: String,
    /// Namespace/Tenant
    #[pyo3(set, get)]
    pub namespace: String,
    /// AppName
    #[pyo3(set, get)]
    pub app_name: Option<String>,
    /// Username for Auth
    #[pyo3(set, get)]
    pub username: Option<String>,
    /// Password for Auth
    #[pyo3(set, get)]
    pub password: Option<String>,
    /// naming push_empty_protection, default true
    #[pyo3(set, get)]
    pub naming_push_empty_protection: Option<bool>,
    /// naming load_cache_at_start, default false
    #[pyo3(set, get)]
    pub naming_load_cache_at_start: Option<bool>,
}

#[pymethods]
impl ClientOptions {
    #[new]
    pub fn new(
        server_addr: String,
        namespace: String,
        app_name: Option<String>,
        username: Option<String>,
        password: Option<String>,
        naming_push_empty_protection: Option<bool>,
        naming_load_cache_at_start: Option<bool>,
    ) -> PyResult<ClientOptions> {
        Ok(Self {
            server_addr,
            namespace,
            app_name,
            username,
            password,
            naming_push_empty_protection,
            naming_load_cache_at_start,
        })
    }

    pub fn __setstate__(&mut self, state: &PyBytes) -> PyResult<()> {
        *self = deserialize(state.as_bytes()).unwrap();
        Ok(())
    }

    pub fn __getstate__<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(py, &serialize(&self).unwrap()))
    }

    /// Used for enabling python pickle library to serialize/deserialize this struct
    /// ref: https://github.com/PyO3/pyo3/issues/100#issuecomment-1220672112
    pub fn __getnewargs__(&self)
        -> PyResult<(
            String,
            String,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<bool>,
            Option<bool>
        )> {
            Ok((self.server_addr.clone(),
                self.namespace.clone(),
                self.app_name.clone(),
                self.username.clone(),
                self.password.clone(),
                self.naming_push_empty_protection,
                self.naming_load_cache_at_start
            ))
    }
}

mod config;
pub use config::*;

mod naming;
pub use naming::*;
