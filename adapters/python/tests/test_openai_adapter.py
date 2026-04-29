import json
import socket
import urllib.error
import unittest
from unittest import mock

from ajentic_adapter.providers import openai_adapter


class _MockHttpResponse:
    def __init__(self, payload: str):
        self._payload = payload

    def read(self):
        return self._payload.encode("utf-8")

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc, tb):
        return False


class OpenAIAdapterTests(unittest.TestCase):
    def sample_request(self):
        return {
            "protocol_version": "0.1.0",
            "run_id": "run_123",
            "candidate_request_id": "cr_123",
            "provider": "openai",
            "model": "gpt-4.1-mini",
            "objective_ref": "obj://1",
            "constraints_ref": "cons://1",
            "domain_ref": "domain://1",
            "input_ref": "Prompt text",
            "timeout_ms": "1000",
            "max_output_bytes": "2048",
        }

    def raw_request(self):
        return "\n".join(f"{key}={value}" for key, value in self.sample_request().items())

    @mock.patch.dict("os.environ", {}, clear=True)
    def test_missing_api_key_maps_to_failed(self):
        status, _, errors = openai_adapter.call_openai(self.sample_request())

        self.assertEqual(status, "FAILED")
        self.assertIn("missing AJENTIC_OPENAI_API_KEY", errors[0])

    @mock.patch.dict("os.environ", {"AJENTIC_OPENAI_API_KEY": "test-key"}, clear=True)
    def test_empty_model_maps_to_failed(self):
        request = self.sample_request()
        request["model"] = ""

        status, _, errors = openai_adapter.call_openai(request)

        self.assertEqual(status, "FAILED")
        self.assertIn("missing openai model", errors[0])

    @mock.patch.dict("os.environ", {"AJENTIC_OPENAI_API_KEY": "test-key"}, clear=True)
    @mock.patch("urllib.request.urlopen")
    def test_success_maps_to_completed_and_preserves_linkage(self, urlopen):
        urlopen.return_value = _MockHttpResponse(json.dumps({"output_text": "hello"}))

        response = openai_adapter.run(self.raw_request())

        self.assertIn("status=COMPLETED", response)
        self.assertIn("run_id=run_123", response)
        self.assertIn("candidate_request_id=cr_123", response)
        self.assertIn("output_text=hello", response)
        self.assertNotIn("test-key", response)
        self.assertNotIn("promotion", response.lower())
        self.assertNotIn("ledger", response.lower())
        self.assertNotIn("audit", response.lower())
        self.assertNotIn("replay", response.lower())

    @mock.patch.dict(
        "os.environ",
        {
            "AJENTIC_OPENAI_API_KEY": "test-key",
            "AJENTIC_OPENAI_MODEL": "configured-model",
        },
        clear=True,
    )
    @mock.patch("urllib.request.urlopen")
    def test_env_model_overrides_request_model(self, urlopen):
        urlopen.return_value = _MockHttpResponse(json.dumps({"output_text": "hello"}))

        openai_adapter.call_openai(self.sample_request())

        request = urlopen.call_args.args[0]
        payload = json.loads(request.data.decode("utf-8"))
        self.assertEqual(payload["model"], "configured-model")

    @mock.patch.dict("os.environ", {"AJENTIC_OPENAI_API_KEY": "test-key"}, clear=True)
    @mock.patch("urllib.request.urlopen")
    def test_timeout_ms_is_passed_to_provider_call(self, urlopen):
        urlopen.return_value = _MockHttpResponse(json.dumps({"output_text": "hello"}))

        openai_adapter.call_openai(self.sample_request())

        _, kwargs = urlopen.call_args
        self.assertEqual(kwargs["timeout"], 1.0)

    @mock.patch.dict("os.environ", {"AJENTIC_OPENAI_API_KEY": "test-key"}, clear=True)
    @mock.patch("urllib.request.urlopen", side_effect=OSError("down"))
    def test_provider_unavailable_maps_to_failed(self, _):
        status, _, errors = openai_adapter.call_openai(self.sample_request())

        self.assertEqual(status, "FAILED")
        self.assertTrue(errors)

    @mock.patch.dict("os.environ", {"AJENTIC_OPENAI_API_KEY": "test-key"}, clear=True)
    @mock.patch("urllib.request.urlopen", side_effect=socket.timeout())
    def test_timeout_maps_to_blocked(self, _):
        status, _, errors = openai_adapter.call_openai(self.sample_request())

        self.assertEqual(status, "BLOCKED")
        self.assertTrue(errors)

    @mock.patch.dict("os.environ", {"AJENTIC_OPENAI_API_KEY": "test-key"}, clear=True)
    @mock.patch("urllib.request.urlopen", side_effect=urllib.error.HTTPError("", 429, "", {}, None))
    def test_http_429_maps_to_blocked(self, _):
        status, _, errors = openai_adapter.call_openai(self.sample_request())

        self.assertEqual(status, "BLOCKED")
        self.assertTrue(errors)

    @mock.patch.dict("os.environ", {"AJENTIC_OPENAI_API_KEY": "test-key"}, clear=True)
    @mock.patch("urllib.request.urlopen")
    def test_malformed_provider_json_maps_to_failed(self, urlopen):
        urlopen.return_value = _MockHttpResponse("not-json")

        status, _, errors = openai_adapter.call_openai(self.sample_request())

        self.assertEqual(status, "FAILED")
        self.assertTrue(errors)

    @mock.patch.dict("os.environ", {"AJENTIC_OPENAI_API_KEY": "test-key"}, clear=True)
    @mock.patch("urllib.request.urlopen")
    def test_provider_error_object_maps_to_failed(self, urlopen):
        urlopen.return_value = _MockHttpResponse(json.dumps({"error": {"message": "nope"}}))

        status, _, errors = openai_adapter.call_openai(self.sample_request())

        self.assertEqual(status, "FAILED")
        self.assertTrue(errors)

    @mock.patch.dict("os.environ", {"AJENTIC_OPENAI_API_KEY": "test-key"}, clear=True)
    @mock.patch("urllib.request.urlopen")
    def test_empty_generated_response_maps_to_failed(self, urlopen):
        urlopen.return_value = _MockHttpResponse(json.dumps({"output_text": "   "}))

        status, _, errors = openai_adapter.call_openai(self.sample_request())

        self.assertEqual(status, "FAILED")
        self.assertTrue(errors)

    @mock.patch.dict("os.environ", {"AJENTIC_OPENAI_API_KEY": "test-key"}, clear=True)
    @mock.patch("urllib.request.urlopen")
    def test_output_over_max_output_bytes_maps_to_failed(self, urlopen):
        request = self.sample_request()
        request["max_output_bytes"] = "4"
        urlopen.return_value = _MockHttpResponse(json.dumps({"output_text": "hello"}))

        status, _, errors = openai_adapter.call_openai(request)

        self.assertEqual(status, "FAILED")
        self.assertEqual(
            errors,
            ["provider response invalid: output exceeds max_output_bytes"],
        )

    @mock.patch.dict("os.environ", {"AJENTIC_OPENAI_API_KEY": "test-key"}, clear=True)
    @mock.patch("urllib.request.urlopen")
    def test_multiline_output_is_escaped_for_line_protocol(self, urlopen):
        urlopen.return_value = _MockHttpResponse(json.dumps({"output_text": "hello\nworld"}))

        response = openai_adapter.run(self.raw_request())

        self.assertIn("output_text=hello\\nworld", response)
        self.assertNotIn("output_text=hello\nworld", response)


if __name__ == "__main__":
    unittest.main()
