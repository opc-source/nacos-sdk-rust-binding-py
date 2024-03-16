#!/usr/bin/python3

import asyncio
import nacos_sdk_rust_binding_py as nacos

client_options = nacos.ClientOptions("0.0.0.0:8848", "love", "simple_app_py", "nacos", "nacos")

# 一般大部分情况下，应用下仅需一个客户端，而且需要长期持有直至应用停止。
# 因为它内部会初始化与服务端的长链接，后续的数据交互及服务变更等订阅，都是实时地通过长链接告知客户端的。
naming_client = nacos.AsyncNacosNamingClient(client_options)


# 自定义服务订阅函数，接受的参数为 `nacos.NacosConfigResponse`
def subscribe_instances(instances: [nacos.NacosServiceInstance]):
    print(f"subscribe_instances,instances={str(instances)}")
    for ins in instances:
        print(f"subscribe_instances,instances[x].ip={ins.ip}")


async def main():
    await asyncio.sleep(1)

    service_name = "todo-service-name"
    group = "dev"
    service_instance = nacos.NacosServiceInstance("127.0.0.1", 8080)

    # 添加服务订阅（对目标 service_name, group 的服务实例变化的监听）
    await naming_client.subscribe(service_name, group, None, subscribe_instances)

    await asyncio.sleep(1)

    # 注册服务实例
    await naming_client.register_instance(service_name, group, service_instance)

    await asyncio.sleep(1)

    # 获取服务实例列表
    get_instances = await naming_client.get_all_instances(service_name, group)

    assert len(get_instances) > 0
    assert get_instances[0].ip == service_instance.ip

    print(f"get_instances={str(get_instances)}")
    for i in get_instances:
        print(f"get_instances[x].ip={i.ip}")

    # 批量注册服务实例，可使前面的配置监听函数被调用
    service_instance2 = nacos.NacosServiceInstance("127.0.0.2", 8080)
    await naming_client.batch_register_instance(service_name, group, [service_instance, service_instance2])

    # 等待一段时间
    await asyncio.sleep(300)

# 运行主任务
asyncio.run(main())
