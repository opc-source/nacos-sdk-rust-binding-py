#!/usr/bin/python3

import time
import nacos_sdk_rust_binding_py as nacos

client_options = nacos.ClientOptions("0.0.0.0:8848", "love", "simple_app_py", "nacos", "nacos")

naming_client = nacos.NacosNamingClient(client_options)

time.sleep(1)

service_name = "todo-service-name"
group = "dev"
service_instance = nacos.NacosServiceInstance("127.0.0.1", 8080)


# 自定义服务订阅函数，接受的参数为 `nacos.NacosConfigResponse`
def subscribe_instances(instances: [nacos.NacosServiceInstance]):
    print(f"subscribe_instances,instances={str(instances)}")
    for ins in instances:
        print(f"subscribe_instances,instances[x].ip={ins.ip}")


# example: 添加配置监听（对目标 data_id, group 配置变化的监听）
naming_client.subscribe(service_name, group, None, subscribe_instances)

time.sleep(1)

# example: 注册服务实例
naming_client.register_instance(service_name, group, service_instance)

time.sleep(1)

# example: 获取服务实例列表
get_instances = naming_client.get_all_instances(service_name, group)

assert len(get_instances) > 0
assert get_instances[0].ip == service_instance.ip

print(f"get_instances={str(get_instances)}")
for i in get_instances:
    print(f"get_instances[x].ip={i.ip}")

# example: 批量服务实例，可使前面的配置监听函数被调用
service_instance2 = nacos.NacosServiceInstance("127.0.0.2", 8080)
naming_client.batch_register_instance(service_name, group, [service_instance, service_instance2])

time.sleep(300)
