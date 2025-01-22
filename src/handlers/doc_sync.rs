use tower_lsp::lsp_types::{DidOpenTextDocumentParams, DidChangeTextDocumentParams};

pub async fn handle_open(params: DidOpenTextDocumentParams) {
    println!("File opened: {}", params.text_document.uri);
}

pub async fn handle_change(params: DidChangeTextDocumentParams) {
    for change in params.content_changes {
        println!("Change: {}", change.text);
    }
}
