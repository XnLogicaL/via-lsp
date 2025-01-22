use tower_lsp::{LspService, Server};
use crate::server::backend::Backend;

mod features;
mod handlers;
mod server;
mod utils;
mod client_helpers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| Backend { client }).finish();
    Server::new(stdin, stdout, socket).serve(service).await;

    Ok(())
}
