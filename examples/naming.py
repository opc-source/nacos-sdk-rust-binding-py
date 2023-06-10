#!/usr/bin/python3

import time
import nacos_sdk_rust_binding_py as nacos

client_options = nacos.ClientOptions("0.0.0.0:8848", "love", "simple_app_py", "nacos", "nacos")

naming_client = nacos.NacosNamingClient(client_options)

time.sleep(1)

service_name = "todo-service-name"
group = "dev"
service_instance = nacos.NacosServiceInstance("127.0.0.1", 8080)

naming_client.register_instance(service_name, group, service_instance)

time.sleep(1)

get_instances = naming_client.get_all_instances(service_name, group)

assert len(get_instances) > 0
assert get_instances[0].ip == service_instance.ip

time.sleep(30)
