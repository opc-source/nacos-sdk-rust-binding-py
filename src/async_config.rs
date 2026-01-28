#![deny(clippy::all)]

use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::types::PyAnyMethods;
use pyo3::{Bound, PyAny, PyErr, PyResult, Python, pyclass, pymethods};
use pyo3_async_runtimes::tokio::future_into_py;

use std::sync::Arc;

use crate::config::{NacosConfigChangeListener, transfer_conf_resp};

/// Async Client api of Nacos Config.
#[pyclass(module = "nacos_sdk_rust_binding_py")]
pub struct AsyncNacosConfigClient {
    inner: nacos_sdk::api::config::ConfigService,
}

#[pymethods]
impl AsyncNacosConfigClient {
    /// Build a Config Client.
    #[new]
    pub fn new(client_options: crate::ClientOptions) -> PyResult<Self> {
        let props = nacos_sdk::api::props::ClientProps::new()
            .server_addr(client_options.server_addr)
            .namespace(client_options.namespace)
            .app_name(
                client_options
                    .app_name
                    .unwrap_or(nacos_sdk::api::constants::UNKNOWN.to_string()),
            );

        // need enable_auth_plugin_http with username & password
        let is_enable_auth_http =
            client_options.username.is_some() && client_options.password.is_some();
        // need enable_auth_plugin_aliyun with access_key & access_secret
        let is_enable_auth_aliyun =
            client_options.access_key.is_some() && client_options.access_secret.is_some();

        let props = if is_enable_auth_http {
            props
                .auth_username(client_options.username.unwrap())
                .auth_password(client_options.password.unwrap())
        } else if is_enable_auth_aliyun {
            props
                .auth_access_key(client_options.access_key.unwrap())
                .auth_access_secret(client_options.access_secret.unwrap())
                .auth_signature_region_id(client_options.signature_region_id.unwrap())
        } else {
            props
        };

        let config_service_builder = if is_enable_auth_http {
            nacos_sdk::api::config::ConfigServiceBuilder::new(props).enable_auth_plugin_http()
        } else if is_enable_auth_aliyun {
            nacos_sdk::api::config::ConfigServiceBuilder::new(props).enable_auth_plugin_aliyun()
        } else {
            nacos_sdk::api::config::ConfigServiceBuilder::new(props)
        };

        let config_service = config_service_builder
            .build()
            .map_err(|nacos_err| PyRuntimeError::new_err(format!("{:?}", &nacos_err)))?;

        Ok(Self {
            inner: config_service,
        })
    }

    /// Get config's content.
    /// If it fails, pay attention to err
    pub fn get_config<'p>(
        &self,
        py: Python<'p>,
        data_id: String,
        group: String,
    ) -> PyResult<Bound<'p, PyAny>> {
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
    ) -> PyResult<Bound<'p, PyAny>> {
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
    ) -> PyResult<Bound<'p, PyAny>> {
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
    ) -> PyResult<Bound<'p, PyAny>> {
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
        listener: Bound<'p, PyAny>, // PyFunction arg: <NacosConfigResponse>
    ) -> PyResult<Bound<'p, PyAny>> {
        if !listener.is_callable() {
            return Err(PyErr::new::<PyValueError, _>(
                "Arg `listener` must be a callable",
            ));
        }
        let listen_wrap = Arc::new(NacosConfigChangeListener {
            func: Arc::new(listener.into()),
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
        listener: Bound<'p, PyAny>, // PyFunction arg: <NacosConfigResponse>
    ) -> PyResult<Bound<'p, PyAny>> {
        future_into_py(py, async { Ok(()) })
    }
}
