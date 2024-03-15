import pickle
import unittest

from nacos_sdk_rust_binding_py import ClientOptions, NacosServiceInstance, NacosConfigResponse


class Test(unittest.TestCase):

    def test_ClientOptions(self):
        options = ClientOptions(server_addr="127.0.0.1:8848", namespace="")

        options_pickle = pickle.dumps(options)
        options_unpickle: ClientOptions = pickle.loads(options_pickle)

        assert options.server_addr == options_unpickle.server_addr
        assert options.namespace == options_unpickle.namespace
        assert options.app_name == options_unpickle.app_name
        assert options.username == options_unpickle.username
        assert options.password == options_unpickle.password
        assert options.naming_load_cache_at_start == options_unpickle.naming_load_cache_at_start
        assert options.naming_push_empty_protection == options_unpickle.naming_push_empty_protection

    def test_NacosServiceInstance(self):
        ins = NacosServiceInstance(ip="127.0.0.1", port=8848)

        ins_pickle = pickle.dumps(ins)
        ins_unpickle: NacosServiceInstance = pickle.loads(ins_pickle)

        assert ins.instance_id == ins_unpickle.instance_id
        assert ins.ip == ins_unpickle.ip
        assert ins.port == ins_unpickle.port
        assert ins.weight == ins_unpickle.weight
        assert ins.healthy == ins_unpickle.healthy
        assert ins.enabled == ins_unpickle.enabled
        assert ins.ephemeral == ins_unpickle.ephemeral
        assert ins.cluster_name == ins_unpickle.cluster_name
        assert ins.service_name == ins_unpickle.service_name
        assert ins.metadata == ins_unpickle.metadata

    def test_NacosConfigResponse(self):
        resp = NacosConfigResponse(
            namespace="namespace",
            data_id="data_id",
            group="group",
            content="content",
            content_type="content_type",
            md5="md5",
        )

        resp_pickle = pickle.dumps(resp)
        resp_unpickle: NacosConfigResponse = pickle.loads(resp_pickle)

        assert resp.namespace == resp_unpickle.namespace
        assert resp.data_id == resp_unpickle.data_id
        assert resp.group == resp_unpickle.group
        assert resp.content == resp_unpickle.content
        assert resp.content_type == resp_unpickle.content_type
        assert resp.md5 == resp_unpickle.md5
