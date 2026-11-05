import importlib.util
import io
import json
import pathlib
import unittest

path=pathlib.Path(__file__).with_name("probe_alerts_in_ua.py")
spec=importlib.util.spec_from_file_location("probe_alerts_in_ua", path)
module=importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)

class FakeResponse:
    def __init__(self, data): self.data=io.BytesIO(data)
    def __enter__(self): return self
    def __exit__(self, *args): return False
    def read(self, limit): return self.data.read(limit)

class Tests(unittest.TestCase):
    def test_record_metadata_only(self):
        def fake(request,timeout):
            self.assertTrue(request.get_header("Authorization").startswith("Bearer "))
            self.assertEqual(timeout,12)
            return FakeResponse(b'[{"regionId": 1}]')
        out=module.receive(fake, "test-token")
        self.assertEqual(out["count"],1)
        self.assertFalse(out["admitted_official"])
        self.assertNotIn("test-token",json.dumps(out))
    def test_invalid_payload_rejected(self):
        with self.assertRaises(ValueError):
            module.receive(lambda req,timeout:FakeResponse(b'"safe"'),"test")
    def test_malformed_json_rejected(self):
        with self.assertRaises(json.JSONDecodeError):
            module.receive(lambda req,timeout:FakeResponse(b'{'),"test")

if __name__=="__main__":unittest.main()
