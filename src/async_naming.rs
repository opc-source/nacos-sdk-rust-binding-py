#![deny(clippy::all)]

use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::{pyclass, pymethods, PyAny, PyErr, PyResult, Python, ToPyObject};
use pyo3_asyncio::tokio::future_into_py;

use std::sync::Arc;

use crate::naming::{
    transfer_ffi_instance_to_rust, transfer_rust_instance_to_ffi, NacosNamingEventListener,
    NacosServiceInstance,
};

/// Async Client api of Nacos Naming.
#[pyclass(module = "nacos_sdk_rust_binding_py")]
pub struct AsyncNacosNamingClient {
    inner: Arc<dyn nacos_sdk::api::naming::NamingService + Send + Sync + 'static>,
}

#[pymethods]
impl AsyncNacosNamingClient {
    /// Build a Naming Client.
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
            )
            .naming_push_empty_protection(
                client_options.naming_push_empty_protection.unwrap_or(true),
            )
            .naming_load_cache_at_start(client_options.naming_load_cache_at_start.unwrap_or(false));

        // need enable_auth_plugin_http with username & password
        let is_enable_auth = client_options.username.is_some() && client_options.password.is_some();

        let props = if is_enable_auth {
            props
                .auth_username(client_options.username.unwrap())
                .auth_password(client_options.password.unwrap())
        } else {
            props
        };

        let naming_service_builder = if is_enable_auth {
            nacos_sdk::api::naming::NamingServiceBuilder::new(props).enable_auth_plugin_http()
        } else {
            nacos_sdk::api::naming::NamingServiceBuilder::new(props)
        };

        let naming_service = naming_service_builder
            .build()
            .map_err(|nacos_err| PyRuntimeError::new_err(format!("{:?}", &nacos_err)))?;

        Ok(Self {
            inner: Arc::new(naming_service),
        })
    }

    /// Register instance.
    /// If it fails, pay attention to err
    pub fn register_instance<'p>(
        &self,
        py: Python<'p>,
        service_name: String,
        group: String,
        service_instance: NacosServiceInstance,
    ) -> PyResult<&'p PyAny> {
        let this = self.inner.clone();
        future_into_py(py, async move {
            this.register_instance(
                service_name,
                Some(group),
                transfer_ffi_instance_to_rust(&service_instance),
            )
            .await
            .map_err(|nacos_err| PyRuntimeError::new_err(format!("{:?}", &nacos_err)))
        })
    }

    /// Deregister instance.
    /// If it fails, pay attention to err
    pub fn deregister_instance<'p>(
        &self,
        py: Python<'p>,
        service_name: String,
        group: String,
        service_instance: NacosServiceInstance,
    ) -> PyResult<&'p PyAny> {
        let this = self.inner.clone();
        future_into_py(py, async move {
            this.deregister_instance(
                service_name,
                Some(group),
                transfer_ffi_instance_to_rust(&service_instance),
            )
            .await
            .map_err(|nacos_err| PyRuntimeError::new_err(format!("{:?}", &nacos_err)))
        })
    }

    /// Batch register instance, improve interaction efficiency.
    /// If it fails, pay attention to err
    pub fn batch_register_instance<'p>(
        &self,
        py: Python<'p>,
        service_name: String,
        group: String,
        service_instances: Vec<NacosServiceInstance>,
    ) -> PyResult<&'p PyAny> {
        let this = self.inner.clone();
        future_into_py(py, async move {
            let rust_instances = service_instances
                .iter()
                .map(transfer_ffi_instance_to_rust)
                .collect();
            this.batch_register_instance(service_name, Some(group), rust_instances)
                .await
                .map_err(|nacos_err| PyRuntimeError::new_err(format!("{:?}", &nacos_err)))
        })
    }

    /// Get all instances by service and group. default cluster=[], subscribe=true.
    /// If it fails, pay attention to err
    pub fn get_all_instances<'p>(
        &self,
        py: Python<'p>,
        service_name: String,
        group: String,
        clusters: Option<Vec<String>>,
        subscribe: Option<bool>,
    ) -> PyResult<&'p PyAny> {
        let this = self.inner.clone();
        future_into_py(py, async move {
            let rust_instances = this
                .get_all_instances(
                    service_name,
                    Some(group),
                    clusters.unwrap_or_default(),
                    subscribe.unwrap_or(true),
                )
                .await
                .map_err(|nacos_err| PyRuntimeError::new_err(format!("{:?}", &nacos_err)))?;

            Ok(rust_instances
                .iter()
                .map(transfer_rust_instance_to_ffi)
                .collect::<Vec<NacosServiceInstance>>())
        })
    }

    /// Select instances whether healthy or not. default cluster=[], subscribe=true, healthy=true.
    /// If it fails, pay attention to err
    pub fn select_instances<'p>(
        &self,
        py: Python<'p>,
        service_name: String,
        group: String,
        clusters: Option<Vec<String>>,
        subscribe: Option<bool>,
        healthy: Option<bool>,
    ) -> PyResult<&'p PyAny> {
        let this = self.inner.clone();
        future_into_py(py, async move {
            let rust_instances = this
                .select_instances(
                    service_name,
                    Some(group),
                    clusters.unwrap_or_default(),
                    subscribe.unwrap_or(true),
                    healthy.unwrap_or(true),
                )
                .await
                .map_err(|nacos_err| PyRuntimeError::new_err(format!("{:?}", &nacos_err)))?;
            Ok(rust_instances
                .iter()
                .map(transfer_rust_instance_to_ffi)
                .collect::<Vec<NacosServiceInstance>>())
        })
    }

    /// Select one healthy instance. default cluster=[], subscribe=true.
    /// If it fails, pay attention to err
    pub fn select_one_healthy_instance<'p>(
        &self,
        py: Python<'p>,
        service_name: String,
        group: String,
        clusters: Option<Vec<String>>,
        subscribe: Option<bool>,
    ) -> PyResult<&'p PyAny> {
        let this = self.inner.clone();
        future_into_py(py, async move {
            let rust_instance = this
                .select_one_healthy_instance(
                    service_name,
                    Some(group),
                    clusters.unwrap_or_default(),
                    subscribe.unwrap_or(true),
                )
                .await
                .map_err(|nacos_err| PyRuntimeError::new_err(format!("{:?}", &nacos_err)))?;
            Ok(transfer_rust_instance_to_ffi(&rust_instance))
        })
    }

    /// Add NacosNamingEventListener callback func, which listen the instance change.
    /// If it fails, pay attention to err
    #[pyo3(signature = (service_name, group, clusters, listener))]
    pub fn subscribe<'p>(
        &self,
        py: Python<'p>,
        service_name: String,
        group: String,
        clusters: Option<Vec<String>>,
        listener: &PyAny, // PyFunction arg: Vec<NacosServiceInstance>
    ) -> PyResult<&'p PyAny> {
        if !listener.is_callable() {
            return Err(PyErr::new::<PyValueError, _>(
                "Arg `listener` must be a callable",
            ));
        }
        let listen_wrap = Arc::new(NacosNamingEventListener {
            func: Arc::new(listener.to_object(py)),
        });
        let this = self.inner.clone();

        future_into_py(py, async move {
            this.subscribe(
                service_name,
                Some(group),
                clusters.unwrap_or_default(),
                listen_wrap,
            )
            .await
            .map_err(|nacos_err| PyRuntimeError::new_err(format!("{:?}", &nacos_err)))?;
            Ok(())
        })
    }

    /// Remove NacosNamingEventListener callback func, but noop....
    /// The logic is not implemented internally, and only APIs are provided as compatibility.
    /// Users maybe do not need it? Not removing the subscription is not a big problem, Sorry!
    #[pyo3(signature = (service_name, group, clusters, listener))]
    #[allow(unused_variables)]
    pub fn un_subscribe<'p>(
        &self,
        py: Python<'p>,
        service_name: String,
        group: String,
        clusters: Option<Vec<String>>,
        listener: &PyAny, // PyFunction arg: Vec<NacosServiceInstance>
    ) -> PyResult<&'p PyAny> {
        future_into_py(py, async move { Ok(()) })
    }
}
