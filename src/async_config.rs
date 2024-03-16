#![deny(clippy::all)]

use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::{pyclass, pymethods, PyAny, PyErr, PyResult, Python, ToPyObject};
use pyo3_asyncio::tokio::future_into_py;

use std::sync::Arc;

use crate::config::{transfer_conf_resp, NacosConfigChangeListener};

/// Async Client api of Nacos Config.
#[pyclass(module = "nacos_sdk_rust_binding_py")]
pub struct AsyncNacosConfigClient {
    inner: Arc<dyn nacos_sdk::api::config::ConfigService + Send + Sync + 'static>,
}

#[pymethods]
impl AsyncNacosConfigClient {
    /// Build a Config Client.
    #[new]
    pub fn new(client_options: crate::ClientOptions) -> PyResult<Self> {
        // print to console or file
        let _ = crate::init_logger();

        let props = nacos_sdk::api::props::ClientProps::new()
            .server_addr(client_options.server_addr)
            .namespace(client_options.namespace)
            .app_name(
                client_options
                    .app_name
                    .unwrap_or(nacos_sdk::api::constants::UNKNOWN.to_string()),
            );

        // need enable_auth_plugin_http with username & password
        let is_enable_auth = client_options.username.is_some() && client_options.password.is_some();

        let props = if is_enable_auth {
            props
                .auth_username(client_options.username.unwrap())
                .auth_password(client_options.password.unwrap())
        } else {
            props
        };

        let config_service_builder = if is_enable_auth {
            nacos_sdk::api::config::ConfigServiceBuilder::new(props).enable_auth_plugin_http()
        } else {
            nacos_sdk::api::config::ConfigServiceBuilder::new(props)
        };

        let config_service = config_service_builder
            .build()
            .map_err(|nacos_err| PyRuntimeError::new_err(format!("{:?}", &nacos_err)))?;

        Ok(Self {
            inner: Arc::new(config_service),
        })
    }

    /// Get config's content.
    /// If it fails, pay attention to err
    pub fn get_config<'p>(
        &self,
        py: Python<'p>,
        data_id: String,
        group: String,
    ) -> PyResult<&'p PyAny> {
        let this = self.inner.clone();
        future_into_py(py, async move {
            let config_resp = this
                .get_config(data_id, group)
                .await
                .map_err(|nacos_err| PyRuntimeError::new_err(format!("{:?}", &nacos_err)))?;
            Ok(transfer_conf_resp(config_resp).content)
        })
    }

    /// Get NacosConfigResponse.
    /// If it fails, pay attention to err
    pub fn get_config_resp<'p>(
        &self,
        py: Python<'p>,
        data_id: String,
        group: String,
    ) -> PyResult<&'p PyAny> {
        let this = self.inner.clone();
        future_into_py(py, async move {
            let config_resp = this
                .get_config(data_id, group)
                .await
                .map_err(|nacos_err| PyRuntimeError::new_err(format!("{:?}", &nacos_err)))?;
            Ok(transfer_conf_resp(config_resp))
        })
    }

    /// Publish config.
    /// If it fails, pay attention to err
    pub fn publish_config<'p>(
        &self,
        py: Python<'p>,
        data_id: String,
        group: String,
        content: String,
    ) -> PyResult<&'p PyAny> {
        let this = self.inner.clone();
        future_into_py(py, async move {
            this.publish_config(data_id, group, content, None)
                .await
                .map_err(|nacos_err| PyRuntimeError::new_err(format!("{:?}", &nacos_err)))
        })
    }

    /// Remove config.
    /// If it fails, pay attention to err
    pub fn remove_config<'p>(
        &self,
        py: Python<'p>,
        data_id: String,
        group: String,
    ) -> PyResult<&'p PyAny> {
        let this = self.inner.clone();
        future_into_py(py, async move {
            this.remove_config(data_id, group)
                .await
                .map_err(|nacos_err| PyRuntimeError::new_err(format!("{:?}", &nacos_err)))
        })
    }

    /// Add NacosConfigChangeListener callback func, which listen the config change.
    /// If it fails, pay attention to err
    #[pyo3(signature = (data_id, group, listener))]
    pub fn add_listener<'p>(
        &self,
        py: Python<'p>,
        data_id: String,
        group: String,
        listener: &PyAny, // PyFunction arg: <NacosConfigResponse>
    ) -> PyResult<&'p PyAny> {
        if !listener.is_callable() {
            return Err(PyErr::new::<PyValueError, _>(
                "Arg `listener` must be a callable",
            ));
        }
        let listen_wrap = Arc::new(NacosConfigChangeListener {
            func: Arc::new(listener.to_object(py)),
        });
        let this = self.inner.clone();
        future_into_py(py, async move {
            this.add_listener(data_id, group, listen_wrap)
                .await
                .map_err(|nacos_err| PyRuntimeError::new_err(format!("{:?}", &nacos_err)))?;
            Ok(())
        })
    }

    /// Remove NacosConfigChangeListener callback func, but noop....
    /// The logic is not implemented internally, and only APIs are provided as compatibility.
    /// Users maybe do not need it? Not removing the listener is not a big problem, Sorry!
    #[pyo3(signature = (data_id, group, listener))]
    #[allow(unused_variables)]
    pub fn remove_listener<'p>(
        &self,
        py: Python<'p>,
        data_id: String,
        group: String,
        listener: &PyAny, // PyFunction arg: <NacosConfigResponse>
    ) -> PyResult<&'p PyAny> {
        future_into_py(py, async { Ok(()) })
    }
}
