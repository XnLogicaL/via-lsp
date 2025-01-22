use tower_lsp::lsp_types::{CompletionParams, CompletionResponse, CompletionItem};

pub async fn handle_completion(_: CompletionParams) -> Option<CompletionResponse> {
    return Some(CompletionResponse::Array(vec![
        CompletionItem::new_simple("example".to_string(), "An example".to_string()),
    ]))
}
