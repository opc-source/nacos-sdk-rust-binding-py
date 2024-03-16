#!/usr/bin/python3

import asyncio
import nacos_sdk_rust_binding_py as nacos

client_options = nacos.ClientOptions("0.0.0.0:8848", "love", "simple_app_py", "nacos", "nacos")

# 一般大部分情况下，应用下仅需一个客户端，而且需要长期持有直至应用停止。
# 因为它内部会初始化与服务端的长链接，后续的数据交互及服务变更等订阅，都是实时地通过长链接告知客户端的。
config_client = nacos.AsyncNacosConfigClient(client_options)


# 自定义配置监听的函数，接受的参数为 `nacos.NacosConfigResponse`
def listen_config(config_resp: nacos.NacosConfigResponse):
    print(f"listen_config,config_resp={str(config_resp)}")
    print(f"listen_config,config_resp.content={config_resp.content}")


async def main():
    await asyncio.sleep(1)

    data_id = "todo-dataid"
    group = "LOVE"
    publish_content = "test-content"

    # 添加配置监听（对目标 data_id, group 配置变化的监听）
    await config_client.add_listener(data_id, group, listen_config)

    # 推送配置
    await config_client.publish_config(data_id, group, publish_content)

    await asyncio.sleep(1)

    # 获取配置，返回值为 `nacos.NacosConfigResponse`
    config_content_resp = await config_client.get_config_resp(data_id, group)

    # 获取配置，返回值为 content: String
    get_config_content = await config_client.get_config(data_id, group)

    assert get_config_content == publish_content
    assert config_content_resp.content == publish_content

    print(f"get_config_content={get_config_content}")
    print(f"config_content_resp={str(config_content_resp)},resp_content={config_content_resp.content}")

    await asyncio.sleep(1)

    # 推送配置，使配置监听函数被调用
    await config_client.publish_config(data_id, group, "publish_content for listen_config")

    # 等待一段时间供用户查看 Nacos 服务器上被监听的配置
    await asyncio.sleep(300)

    # 删除配置
    await config_client.remove_config(data_id, group)

    # 获取已删除的配置，会抛出异常
    try:
        get_config_content_removed = await config_client.get_config(data_id, group)
    except RuntimeError:
        print("config already be removed.")

    await asyncio.sleep(10)

# 运行主任务
asyncio.run(main())
