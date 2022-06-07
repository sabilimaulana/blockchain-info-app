use blockchain_status::BlockchainStatus;

#[macro_use]
extern crate serde_derive;

mod blockchain_address;
mod blockchain_info;
mod blockchain_status;
mod blockchain_transaction;

fn main() {
    let blockchain_status: BlockchainStatus = blockchain_info::blockchain_status_request();
    print!(
        "\n\nQuerying {} - chain: {}\n\n",
        &blockchain_status.blockbook.coin, &blockchain_status.backend.chain
    )
}
