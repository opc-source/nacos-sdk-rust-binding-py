use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn nacos_sdk_rust_binding_py(m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, &m)?)?;
    m.add_class::<ClientOptions>()?;
    m.add_class::<NacosConfigClient>()?;
    m.add_class::<NacosConfigResponse>()?;
    m.add_class::<NacosNamingClient>()?;
    m.add_class::<NacosServiceInstance>()?;
    // Async Client api
    m.add_class::<AsyncNacosConfigClient>()?;
    m.add_class::<AsyncNacosNamingClient>()?;
    Ok(())
}

#[pyclass(module = "nacos_sdk_rust_binding_py")]
#[derive(Clone)]
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
    /// Username for Auth, Login by Http with Token
    #[pyo3(set, get)]
    pub username: Option<String>,
    /// Password for Auth, Login by Http with Token
    #[pyo3(set, get)]
    pub password: Option<String>,
    /// Access_Key for Auth, Login by Aliyun Ram
    #[pyo3(set, get)]
    pub access_key: Option<String>,
    /// Access_Secret for Auth, Login by Aliyun Ram
    #[pyo3(set, get)]
    pub access_secret: Option<String>,
    /// Signature_Region_Id for Auth, Login by Aliyun Ram
    #[pyo3(set, get)]
    pub signature_region_id: Option<String>,
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
    #[pyo3(signature = (server_addr, namespace, app_name=None, username=None, password=None, access_key=None, access_secret=None, signature_region_id=None, naming_push_empty_protection=None, naming_load_cache_at_start=None))]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        server_addr: String,
        namespace: String,
        app_name: Option<String>,
        username: Option<String>,
        password: Option<String>,
        access_key: Option<String>,
        access_secret: Option<String>,
        signature_region_id: Option<String>,
        naming_push_empty_protection: Option<bool>,
        naming_load_cache_at_start: Option<bool>,
    ) -> PyResult<ClientOptions> {
        Ok(Self {
            server_addr,
            namespace,
            app_name,
            username,
            password,
            access_key,
            access_secret,
            signature_region_id,
            naming_push_empty_protection,
            naming_load_cache_at_start,
        })
    }
}

mod config;
pub use config::*;

mod naming;
pub use naming::*;

mod async_config;
pub use async_config::*;

mod async_naming;
pub use async_naming::*;
