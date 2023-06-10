
import nacos_sdk_rust_binding_py

client_options = nacos_sdk_rust_binding_py.ClientOptions("0.0.0.0:8848", "love", "simple_app_py", "nacos", "nacos")

print(client_options)

config_client = nacos_sdk_rust_binding_py.NacosConfigClient(client_options)

config_content = config_client.get_config("todo-dataid", "LOVE")

print(config_content)
