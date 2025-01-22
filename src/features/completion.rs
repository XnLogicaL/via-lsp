use tower_lsp::lsp_types::{CompletionItem, CompletionParams, CompletionResponse, Position, TextDocumentPositionParams};

macro_rules! new_simple {
    ($a:expr, $b:expr) => {
        CompletionItem::new_simple($a.to_string(), $b.to_string())
    };
}

pub async fn handle_completion(params: CompletionParams) -> Option<CompletionResponse> {
    let position: Position = params.text_document_position.position;
    let uri = params.text_document_position.text_document.uri;
    let document_content = ""; // get_document_content(&uri).await;

    let line_content = document_content
        .lines()
        .nth(position.line as usize)
        .unwrap_or("");

    let completions;
    
    if line_content.trim().ends_with("local") || line_content.trim().ends_with("global") {
        completions = vec![
            new_simple!("const", ""),
            new_simple!("func", ""),
        ];
    }
    else if line_content.trim().ends_with("func") {
        completions = vec![
            new_simple!("strict", ""),
            new_simple!("const", ""),
        ];
    }
    else if line_content.trim().ends_with("strict") {
        completions = vec![
            new_simple!("const", "")
        ];
    }
    else if line_content.trim().ends_with("const") {
        completions = vec![
            new_simple!("strict", "")
        ];
    }
    else {
        completions = vec![
            new_simple!("local", ""),
            new_simple!("global", ""),
            new_simple!("if", ""),
            new_simple!("while", ""),
            new_simple!("for", ""),
            new_simple!("return", ""),
            new_simple!("func", ""),
            new_simple!("break", ""),
            new_simple!("continue", ""),
            new_simple!("switch", ""),
            new_simple!("struct", ""),
            new_simple!("namespace", ""),
            new_simple!("import", ""),
            new_simple!("export", ""),
            new_simple!("macro", ""),
            new_simple!("define", ""),
            new_simple!("type", ""),
            new_simple!("typeof", ""),
            new_simple!("defined", ""),
        ]
    };

    return Some(CompletionResponse::Array(completions))
}
