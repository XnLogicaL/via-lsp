use tower_lsp::lsp_types::{Diagnostic, PublishDiagnosticsParams};

pub async fn publish_diagnostics(uri: &str, message: &str) -> PublishDiagnosticsParams {
    PublishDiagnosticsParams {
        uri: uri.parse().unwrap(),
        diagnostics: vec![Diagnostic {
            message: message.to_string(),
            ..Default::default()
        }],
        version: None,
    }
}
