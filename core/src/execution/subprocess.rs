//! Subprocess runner for Phase 4 deterministic adapter invocation.

use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::errors::AdapterProtocolError;
use crate::execution::adapter_protocol::{
    build_adapter_request_text, parse_adapter_response_text, validate_adapter_response,
    AdapterRequest, AdapterResponse,
};

pub fn run_mock_adapter(
    python_executable: &str,
    adapter_script_path: &Path,
    request: &AdapterRequest,
) -> Result<AdapterResponse, AdapterProtocolError> {
    let mut child = Command::new(python_executable)
        .arg(adapter_script_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| AdapterProtocolError::SubprocessFailed)?;

    let request_text = build_adapter_request_text(request);

    if let Some(stdin) = child.stdin.as_mut() {
        stdin
            .write_all(request_text.as_bytes())
            .map_err(|_| AdapterProtocolError::SubprocessFailed)?;
    }

    let output = child
        .wait_with_output()
        .map_err(|_| AdapterProtocolError::SubprocessFailed)?;

    if !output.status.success() {
        return Err(AdapterProtocolError::SubprocessFailed);
    }

    if output.stdout.is_empty() {
        return Err(AdapterProtocolError::SubprocessNoOutput);
    }

    let text = String::from_utf8(output.stdout).map_err(|_| AdapterProtocolError::ParseFailed)?;
    if text.trim().is_empty() {
        return Err(AdapterProtocolError::SubprocessNoOutput);
    }

    let response = parse_adapter_response_text(&text)?;
    validate_adapter_response(request, &response)?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicUsize, Ordering};

    use crate::execution::adapter_protocol::{
        parse_adapter_response_text, AdapterLimits, AdapterRequest, AdapterStatus,
        PHASE4_PROTOCOL_VERSION,
    };

    use super::run_mock_adapter;

    static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn sample_request() -> AdapterRequest {
        AdapterRequest {
            protocol_version: PHASE4_PROTOCOL_VERSION.to_string(),
            run_id: "run_subprocess_001".to_string(),
            candidate_request_id: "candidate_request_subprocess_001".to_string(),
            provider: "mock".to_string(),
            model: "mock-v1".to_string(),
            objective_ref: "objective://mock".to_string(),
            constraints_ref: "constraints://mock".to_string(),
            domain_ref: "domain://mock".to_string(),
            input_ref: "input://mock".to_string(),
            limits: AdapterLimits {
                timeout_ms: 1000,
                max_output_bytes: 1024,
            },
        }
    }

    fn python_executable() -> String {
        env::var("PYTHON").unwrap_or_else(|_| "python3".to_string())
    }

    fn mock_script_path() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("adapters")
            .join("python")
            .join("ajentic_adapter")
            .join("providers")
            .join("mock_adapter.py")
    }

    fn temp_script_path(name: &str) -> PathBuf {
        let index = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        env::temp_dir().join(format!(
            "ajentic_phase4_{}_{}_{}.py",
            name,
            std::process::id(),
            index
        ))
    }

    #[test]
    fn rust_can_call_mock_python_adapter_successfully() {
        let request = sample_request();
        let response =
            run_mock_adapter(&python_executable(), &mock_script_path(), &request).unwrap();

        assert_eq!(response.status, AdapterStatus::Completed);
        assert!(response.output_text.contains("untrusted"));
    }

    #[test]
    fn mock_adapter_output_is_parsed() {
        let request = sample_request();
        let response =
            run_mock_adapter(&python_executable(), &mock_script_path(), &request).unwrap();

        assert_eq!(response.run_id, request.run_id);
        assert_eq!(response.candidate_request_id, request.candidate_request_id);
    }

    #[test]
    fn mock_adapter_output_remains_untrusted() {
        let request = sample_request();
        let response =
            run_mock_adapter(&python_executable(), &mock_script_path(), &request).unwrap();

        assert!(response.output_text.contains("untrusted"));
        assert!(!response
            .output_text
            .to_ascii_lowercase()
            .contains("promoted"));
    }

    #[test]
    fn mock_adapter_cannot_set_lifecycle_state() {
        let bad_response = "protocol_version=0.1.0\n\
adapter_name=mock\n\
adapter_version=0.1.0\n\
run_id=run_subprocess_001\n\
candidate_request_id=candidate_request_subprocess_001\n\
status=COMPLETED\n\
raw_output_ref=mock://raw/run\n\
structured_output_ref=mock://structured/run\n\
output_text=untrusted\n\
lifecycle_state=PromotedTier1\n";

        let error = parse_adapter_response_text(bad_response).unwrap_err();
        assert_eq!(error, AdapterProtocolError::ParseFailed);
    }

    #[test]
    fn mock_adapter_cannot_emit_promotion_approval() {
        let bad_response = "protocol_version=0.1.0\n\
adapter_name=mock\n\
adapter_version=0.1.0\n\
run_id=run_subprocess_001\n\
candidate_request_id=candidate_request_subprocess_001\n\
status=COMPLETED\n\
raw_output_ref=mock://raw/run\n\
structured_output_ref=mock://structured/run\n\
output_text=untrusted\n\
promotion_approval=true\n";

        let error = parse_adapter_response_text(bad_response).unwrap_err();
        assert_eq!(error, AdapterProtocolError::ParseFailed);
    }

    #[test]
    fn missing_adapter_script_path_fails() {
        let request = sample_request();
        let missing = Path::new(env!("CARGO_MANIFEST_DIR")).join("missing_mock_adapter.py");

        let error = run_mock_adapter(&python_executable(), &missing, &request).unwrap_err();
        assert_eq!(error, AdapterProtocolError::SubprocessFailed);
    }

    #[test]
    fn non_zero_subprocess_exit_fails() {
        let request = sample_request();
        let path = temp_script_path("non_zero_exit");
        fs::write(&path, "import sys\nsys.exit(7)\n").unwrap();

        let result = run_mock_adapter(&python_executable(), &path, &request);
        let _ = fs::remove_file(&path);

        let error = result.unwrap_err();
        assert_eq!(error, AdapterProtocolError::SubprocessFailed);
    }

    #[test]
    fn empty_subprocess_output_fails() {
        let request = sample_request();
        let path = temp_script_path("empty_output");
        fs::write(&path, "import sys\n_ = sys.stdin.read()\n").unwrap();

        let result = run_mock_adapter(&python_executable(), &path, &request);
        let _ = fs::remove_file(&path);

        let error = result.unwrap_err();
        assert_eq!(error, AdapterProtocolError::SubprocessNoOutput);
    }

    use crate::errors::AdapterProtocolError;
}
