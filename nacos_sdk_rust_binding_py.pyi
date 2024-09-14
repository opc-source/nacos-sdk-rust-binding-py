from typing import Callable, Dict, List, NoReturn, Optional

class ClientOptions:
    def __init__(
        self,
        server_addr: str,
        namespace: str,
        app_name: Optional[str],
        username: Optional[str],
        password: Optional[str],
        access_key: Optional[str],
        access_secret: Optional[str],
        signature_region_id: Optional[str],
        naming_push_empty_protection: Optional[bool],
        naming_load_cache_at_start: Optional[bool],
    ) -> None: ...

class NacosConfigResponse:
    @property
    def namespace(self) -> str: ...
    @property
    def data_id(self) -> str: ...
    @property
    def group(self) -> str: ...
    @property
    def content(self) -> str: ...
    @property
    def content_type(self) -> str: ...
    @property
    def md5(self) -> str: ...

class NacosConfigClient:
    def __init__(self, client_options: ClientOptions) -> None: ...
    def get_config(self, data_id: str, group: str) -> str:
        """Get config's content. If it fails, pay attention to err"""

        ...
    def get_config_resp(self, data_id: str, group: str) -> NacosConfigResponse:
        """Get NacosConfigResponse. If it fails, pay attention to err"""

        ...
    def publish_config(self, data_id: str, group: str, content: str) -> bool:
        """Publish config. If it fails, pay attention to err"""

        ...
    def remove_config(self, data_id: str, group: str) -> bool:
        """Remove config. If it fails, pay attention to err"""

        ...
    def add_listener(
        self,
        data_id: str,
        group: str,
        listener: Callable[[NacosConfigResponse], NoReturn],
    ):
        """Add NacosConfigChangeListener callback func, which listen the config change. If it fails, pay attention to err"""

        ...

class AsyncNacosConfigClient:
    def __init__(self, client_options: ClientOptions) -> None: ...
    async def get_config(self, data_id: str, group: str) -> str:
        """Get config's content. If it fails, pay attention to err"""

        ...
    async def get_config_resp(self, data_id: str, group: str) -> NacosConfigResponse:
        """Get NacosConfigResponse. If it fails, pay attention to err"""

        ...
    async def publish_config(self, data_id: str, group: str, content: str) -> bool:
        """Publish config. If it fails, pay attention to err"""

        ...
    async def remove_config(self, data_id: str, group: str) -> bool:
        """Remove config. If it fails, pay attention to err"""

        ...
    async def add_listener(
        self,
        data_id: str,
        group: str,
        listener: Callable[[NacosConfigResponse], NoReturn],
    ):
        """Add NacosConfigChangeListener callback func, which listen the config change. If it fails, pay attention to err"""

        ...

class NacosServiceInstance:
    def __init__(
        self,
        ip: str,
        port: int,
        weight: Optional[float],
        healthy: Optional[bool],
        enabled: Optional[bool],
        ephemeral: Optional[bool],
        cluster_name: Optional[str],
        service_name: Optional[str],
        metadata: Optional[Dict[str, str]],
    ) -> None: ...

class NacosNamingClient:
    def __init__(self, client_options: ClientOptions) -> None: ...
    def register_instance(
        self, service_name: str, group: str, service_instance: NacosServiceInstance
    ):
        """Register instance. If it fails, pay attention to err"""

        ...

    def deregister_instance(
        self, service_name: str, group: str, service_instance: NacosServiceInstance
    ):
        """Deregister instance. If it fails, pay attention to err"""

        ...

    def batch_register_instance(
        self,
        service_name: str,
        group: str,
        service_instances: List[NacosServiceInstance],
    ):
        """Batch register instance, improve interaction efficiency. If it fails, pay attention to err"""

        ...

    def get_all_instances(
        self,
        service_name: str,
        group: str,
        clusters: Optional[List[str]],
        subscribe: Optional[bool],
    ) -> List[NacosServiceInstance]:
        """Get all instances by service and group. default cluster=[], subscribe=true. If it fails, pay attention to err"""

        ...

    def select_instances(
        self,
        service_name: str,
        group: str,
        clusters: Optional[List[str]],
        subscribe: Optional[bool],
        healthy: Optional[bool],
    ) -> List[NacosServiceInstance]:
        """Select instances whether healthy or not. default cluster=[], subscribe=true, healthy=true. If it fails, pay attention to err"""

        ...

    def select_one_healthy_instance(
        self,
        service_name: str,
        group: str,
        clusters: Optional[List[str]],
        subscribe: Optional[bool],
    ) -> NacosServiceInstance:
        """Select one healthy instance. default cluster=[], subscribe=true. If it fails, pay attention to err"""

        ...

    def subscribe(
        self,
        service_name: str,
        group: str,
        clusters: Optional[List[str]],
        listener: Callable[[NacosConfigResponse], NoReturn],
    ) -> NacosServiceInstance:
        """Add NacosNamingEventListener callback func, which listen the instance change. If it fails, pay attention to err"""

        ...

class AsyncNacosNamingClient:
    def __init__(self, client_options: ClientOptions) -> None: ...
    async def register_instance(
        self, service_name: str, group: str, service_instance: NacosServiceInstance
    ):
        """Register instance. If it fails, pay attention to err"""

        ...

    async def deregister_instance(
        self, service_name: str, group: str, service_instance: NacosServiceInstance
    ):
        """Deregister instance. If it fails, pay attention to err"""

        ...

    async def batch_register_instance(
        self,
        service_name: str,
        group: str,
        service_instances: List[NacosServiceInstance],
    ):
        """Batch register instance, improve interaction efficiency. If it fails, pay attention to err"""

        ...

    async def get_all_instances(
        self,
        service_name: str,
        group: str,
        clusters: Optional[List[str]],
        subscribe: Optional[bool],
    ) -> List[NacosServiceInstance]:
        """Get all instances by service and group. default cluster=[], subscribe=true. If it fails, pay attention to err"""

        ...

    async def select_instances(
        self,
        service_name: str,
        group: str,
        clusters: Optional[List[str]],
        subscribe: Optional[bool],
        healthy: Optional[bool],
    ) -> List[NacosServiceInstance]:
        """Select instances whether healthy or not. default cluster=[], subscribe=true, healthy=true. If it fails, pay attention to err"""

        ...

    async def select_one_healthy_instance(
        self,
        service_name: str,
        group: str,
        clusters: Optional[List[str]],
        subscribe: Optional[bool],
    ) -> NacosServiceInstance:
        """Select one healthy instance. default cluster=[], subscribe=true. If it fails, pay attention to err"""

        ...

    async def subscribe(
        self,
        service_name: str,
        group: str,
        clusters: Optional[List[str]],
        listener: Callable[[NacosConfigResponse], NoReturn],
    ) -> NacosServiceInstance:
        """Add NacosNamingEventListener callback func, which listen the instance change. If it fails, pay attention to err"""

        ...
