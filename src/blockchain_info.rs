use crate::blockchain_address::BlockchainAddress;
use crate::blockchain_status::BlockchainStatus;
use crate::blockchain_transaction::BlockchainTransaction;
use dotenv;
use reqwest;
use serde_json::Result;
use tokio;

const HOST_ROOT: &str = "https://btcbook.nownodes.io/api/";

#[tokio::main]
pub async fn send_request(url: &str) -> String {
    let client = reqwest::Client::new();
    client
        .get(url)
        .header(
            "api-key",
            dotenv::var("API_KEY").expect("Could not find key: API_KEY"),
        )
        .send()
        .await
        .expect("Failed to get response")
        .text()
        .await
        .expect("Failed to convert payload")
}

pub fn blockchain_status_request() -> BlockchainStatus {
    let response = send_request(&HOST_ROOT);
    return serde_json::from_str(&response).expect("Failed to parse JSON");
}
