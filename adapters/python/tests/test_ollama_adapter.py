import json
import socket
import unittest
from unittest import mock

from ajentic_adapter.providers import ollama_adapter


class _MockHttpResponse:
    def __init__(self, payload: str):
        self._payload = payload

    def read(self):
        return self._payload.encode("utf-8")

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc, tb):
        return False


class OllamaAdapterTests(unittest.TestCase):
    def sample_request(self):
        return {
            "protocol_version": "0.1.0",
            "run_id": "run_123",
            "candidate_request_id": "cr_123",
            "provider": "ollama",
            "model": "tiny",
            "objective_ref": "obj://1",
            "constraints_ref": "cons://1",
            "domain_ref": "domain://1",
            "input_ref": "Prompt text",
            "timeout_ms": "1000",
            "max_output_bytes": "2048",
        }

    @mock.patch("urllib.request.urlopen")
    def test_success_maps_to_completed_and_preserves_linkage(self, urlopen):
        urlopen.return_value = _MockHttpResponse(json.dumps({"response": "hello"}))
        response = ollama_adapter.run("\n".join(f"{k}={v}" for k, v in self.sample_request().items()))
        self.assertIn("status=COMPLETED", response)
        self.assertIn("run_id=run_123", response)
        self.assertIn("candidate_request_id=cr_123", response)
        self.assertNotIn("promotion", response.lower())
        self.assertNotIn("ledger", response.lower())
        self.assertNotIn("audit", response.lower())

    @mock.patch("urllib.request.urlopen", side_effect=socket.timeout())
    def test_timeout_maps_to_blocked(self, _):
        status, _, errors = ollama_adapter.call_ollama(self.sample_request())
        self.assertEqual(status, "BLOCKED")
        self.assertTrue(errors)

    @mock.patch("urllib.request.urlopen", side_effect=OSError("down"))
    def test_provider_unavailable_maps_to_failed(self, _):
        status, _, errors = ollama_adapter.call_ollama(self.sample_request())
        self.assertEqual(status, "FAILED")
        self.assertTrue(errors)

    @mock.patch("urllib.request.urlopen")
    def test_malformed_provider_json_maps_to_failed(self, urlopen):
        urlopen.return_value = _MockHttpResponse("not-json")
        status, _, errors = ollama_adapter.call_ollama(self.sample_request())
        self.assertEqual(status, "FAILED")
        self.assertTrue(errors)

    @mock.patch("urllib.request.urlopen")
    def test_empty_generated_response_maps_to_failed(self, urlopen):
        urlopen.return_value = _MockHttpResponse(json.dumps({"response": "   "}))
        status, _, errors = ollama_adapter.call_ollama(self.sample_request())
        self.assertEqual(status, "FAILED")
        self.assertTrue(errors)


if __name__ == "__main__":
    unittest.main()
