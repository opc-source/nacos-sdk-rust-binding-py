# nacos-sdk-rust-binding-py
nacos-sdk-rust binding for Python with PyO3.

Tip: nacos-sdk-python 仓库暂未提供 2.x gRPC 交互模式，为了能升级它，故而通过 ffi 方式调用 nacos-sdk-rust

## Installation

```bash
pip install nacos-sdk-rust-binding-py
```

- project package see https://pypi.org/project/nacos-sdk-rust-binding-py

## Usage

**使用样例请看仓库内的 examples 目录**

环境变量 `NACOS_CLIENT_LOGGER_LEVEL=INFO` 可设置日志打印级别，默认 INFO
- 客户端日志请在目录 `$HOME/logs/nacos/` 查看

环境变量 `NACOS_CLIENT_COMMON_THREAD_CORES=4` 可设置客户端核心线程数，默认是 CPU 数目 num_cpus

### Definition of ClientOptions

```python
class ClientOptions:
    # Server Addr, e.g. address:port[,address:port],...]
    #[pyo3(set, get)]
    server_addr: String,
    # Namespace/Tenant
    #[pyo3(set, get)]
    namespace: String,
    # AppName
    #[pyo3(set, get)]
    app_name: Option<String>,
    # Username for Auth
    #[pyo3(set, get)]
    username: Option<String>,
    # Password for Auth
    #[pyo3(set, get)]
    password: Option<String>,

    # Init
    def __init__(self, server_addr, namespace, app_name, username, password):
        self.server_addr = server_addr
        self.server_addr = namespace
        self.app_name = app_name
        self.username = username
        self.password = password

```

### Definition of Config

```python
class NacosConfigResponse:
    # Namespace/Tenant
    # [pyo3(get)]
    namespace: String,
    # DataId
    # [pyo3(get)]
    data_id: String,
    # Group
    # [pyo3(get)]
    group: String,
    # Content
    # [pyo3(get)]
    content: String,
    # Content's Type; e.g. json,properties,xml,html,text,yaml
    # [pyo3(get)]
    content_type: String,
    # Content's md5
    # [pyo3(get)]
    md5: String,


class NacosConfigClient:
    # Init. If it fails, pay attention to err
    def __init__(self, client_options: ClientOptions):
        # inner logic xxx
        pass

    # Get config's content. If it fails, pay attention to err
    def get_config(self, data_id: String, group: String) -> String:
        pass

    # Get NacosConfigResponse. If it fails, pay attention to err
    def get_config_resp(self, data_id: String, group: String) -> NacosConfigResponse:
        pass

    # Publish config. If it fails, pay attention to err
    def publish_config(self, data_id: String, group: String, content: String) -> bool:
        pass

    # Remove config. If it fails, pay attention to err
    def remove_config(self, data_id: String, group: String) -> bool:
        pass

    # Add NacosConfigChangeListener callback func, which listen the config change. If it fails, pay attention to err
    def add_listener(self, data_id: String, group: String, listener: py_function):
        pass


```

### Definition of Naming

```python
class NacosServiceInstance:
    # Instance Id
    #[pyo3(set, get)]
    instance_id: Option<String>,
    # Ip
    #[pyo3(set, get)]
    ip: String,
    # Port
    #[pyo3(set, get)]
    port: i32,
    # Weight, default 1.0
    #[pyo3(set, get)]
    weight: Option<f64>,
    # Healthy or not, default true
    #[pyo3(set, get)]
    healthy: Option<bool>,
    # Enabled ot not, default true
    #[pyo3(set, get)]
    enabled: Option<bool>,
    # Ephemeral or not, default true
    #[pyo3(set, get)]
    ephemeral: Option<bool>,
    # Cluster Name, default 'DEFAULT'
    #[pyo3(set, get)]
    cluster_name: Option<String>,
    # Service Name
    #[pyo3(set, get)]
    service_name: Option<String>,
    # Metadata, default '{}'
    #[pyo3(set, get)]
    metadata: Option<std::collections::HashMap<String, String>>,

    # Init
    def __init__(self, ip, port, weight, healthy, enabled, ephemeral, cluster_name, service_name, metadata):
        # inner logic xxx
        pass


class NacosNamingClient:
    # Init. If it fails, pay attention to err
    def __init__(self, client_options: ClientOptions):
        # inner logic xxx
        pass

    # Register instance. If it fails, pay attention to err
    def register_instance(self, service_name: String, group: String, service_instance: NacosServiceInstance):
        pass

    # Deregister instance. If it fails, pay attention to err
    def deregister_instance(self, service_name: String, group: String, service_instance: NacosServiceInstance):
        pass

    # Batch register instance, improve interaction efficiency. If it fails, pay attention to err
    def batch_register_instance(self, service_name: String, group: String, service_instances: [NacosServiceInstance]):
        pass

    # Get all instances by service and group. default cluster=[], subscribe=true. If it fails, pay attention to err
    def get_all_instances(self, service_name: String, group: String, clusters: Option<[String]>, subscribe: Option<bool>) -> [NacosServiceInstance]:
        pass

    # Select instances whether healthy or not. default cluster=[], subscribe=true, healthy=true. If it fails, pay attention to err
    def select_instances(self, service_name: String, group: String, clusters: Option<[String]>, subscribe: Option<bool>, healthy: Option<bool>) -> [NacosServiceInstance]:
        pass

    # Select one healthy instance. default cluster=[], subscribe=true. If it fails, pay attention to err
    def select_one_healthy_instance(self, service_name: String, group: String, clusters: Option<[String]>, subscribe: Option<bool>) -> NacosServiceInstance:
        pass

    # Add NacosNamingEventListener callback func, which listen the instance change. If it fails, pay attention to err
    def subscribe(self, service_name: String, group: String, clusters: Option<[String]>, listener: py_function) -> NacosServiceInstance:
        pass


```

## Development

Setup virtualenv:

```shell
python -m venv venv
```

Activate venv:

```shell
source venv/bin/activate
````

Install `maturin`:

```shell
pip install maturin[patchelf]
```

Build bindings:

```shell
maturin develop
```

Run some tests:

```shell
maturin develop -E test
behave tests
```

Build API docs:

```shell
maturin develop -E docs
pdoc nacos-sdk-rust-binding-py
```

# License
[Apache License Version 2.0](LICENSE)

# Acknowledgement
- binding for Python with [PyO3](https://github.com/PyO3/pyo3.git)
- binding the [nacos-sdk-rust](https://github.com/nacos-group/nacos-sdk-rust.git)
