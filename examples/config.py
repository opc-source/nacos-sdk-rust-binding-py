#!/usr/bin/python3

import time
import nacos_sdk_rust_binding_py as nacos

client_options = nacos.ClientOptions("0.0.0.0:8848", "love", "simple_app_py", "nacos", "nacos")

config_client = nacos.NacosConfigClient(client_options)

time.sleep(1)

data_id = "todo-dataid"
group = "LOVE"
publish_content = "test-content"


# 自定义配置监听的函数，接受的参数为 `nacos.NacosConfigResponse`
def listen_config(config_resp: nacos.NacosConfigResponse):
    print(f"listen_config,config_resp={str(config_resp)}")
    print(f"listen_config,config_resp.content={config_resp.content}")


# example: 添加配置监听（对目标 data_id, group 配置变化的监听）
config_client.add_listener(data_id, group, listen_config)

# example: 推送配置
config_client.publish_config(data_id, group, publish_content)

time.sleep(1)

# example: 获取配置，返回值为 `nacos.NacosConfigResponse`
config_content_resp = config_client.get_config_resp(data_id, group)

# example: 获取配置，返回值为 content: String
get_config_content = config_client.get_config(data_id, group)

assert get_config_content == publish_content
assert config_content_resp.content == publish_content

print(f"get_config_content={get_config_content}")
print(f"config_content_resp={str(config_content_resp)},resp_content={config_content_resp.content}")

time.sleep(1)

# example: 推送配置，使配置监听函数被调用
config_client.publish_config(data_id, group, "publish_content for listen_config")

# sleep for user look at nacos-server, the config be listening
time.sleep(300)

# example: 删除配置
config_client.remove_config(data_id, group)

# example: 获取的配置不存在，会抛出异常
try:
    get_config_content_removed = config_client.get_config(data_id, group)
except RuntimeError:
    print("config already be removed.")

time.sleep(10)
