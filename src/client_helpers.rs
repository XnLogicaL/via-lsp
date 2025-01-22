use tower_lsp::{Client, lsp_types::MessageType};

pub async fn log_info(client: &Client, message: &str) {
    client.log_message(MessageType::INFO, message).await;
}
