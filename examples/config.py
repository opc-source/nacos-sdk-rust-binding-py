#!/usr/bin/python3

import time
import nacos_sdk_rust_binding_py as nacos

client_options = nacos.ClientOptions("0.0.0.0:8848", "love", "simple_app_py", "nacos", "nacos")

config_client = nacos.NacosConfigClient(client_options)

time.sleep(1)

data_id = "todo-dataid"
group = "LOVE"
publish_content = "test-content"

config_client.publish_config(data_id, group, publish_content)

time.sleep(1)

config_content_resp = config_client.get_config_resp(data_id, group)

get_config_content = config_client.get_config(data_id, group)

assert get_config_content == publish_content
assert config_content_resp.content == publish_content

print("get_config_content=" + get_config_content)
print("config_content_resp.content=" + config_content_resp.content)

config_client.remove_config(data_id, group)

try:
    get_config_content_removed = config_client.get_config(data_id, group)
except RuntimeError:
    print("config already be removed.")

time.sleep(10)
