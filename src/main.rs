#[macro_use]
extern crate serde_derive;

mod blockchain_address;
mod blockchain_info;
mod blockchain_status;
mod blockchain_transaction;

use crate::{
    blockchain_address::BlockchainAddress, blockchain_status::BlockchainStatus,
    blockchain_transaction::BlockchainTransaction,
};

use std::{io, thread, time};

fn blockchain_info_app(address: &str) {
    let blockchain_status: BlockchainStatus = blockchain_info::blockchain_status_request();
    println!(
        "\n\nQuerying {} - chain: {}\n\n",
        &blockchain_status.blockbook.coin, &blockchain_status.backend.chain
    );

    let blockchain_address: BlockchainAddress =
        blockchain_info::blockchain_address_request(address);
    println!(
        "\n\nAnalysing transactions for Bitcoin address {}",
        &blockchain_address.address
    );

    let sleep_time = time::Duration::from_millis(2500);
    thread::sleep(sleep_time);

    println!(
        "\nYou have a total of {} transactions",
        &blockchain_address.txids.len()
    );

    println!("\nDo you want to see the transactions? (y/n)");

    let mut command = String::new();
    io::stdin()
        .read_line(&mut command)
        .expect("Input doesn't valid");

    if command.trim().to_lowercase().eq("y") {
        println!("\nWe will look up the transactions:\n");
        println!("{:#?}", &blockchain_address.txids);
        thread::sleep(sleep_time);

        let mut balance: i32 = 0;
        for tx_id in &blockchain_address.txids {
            let mut subtotal_vin = 0;
            let mut subtotal_vout = 0;

            let blockchain_transaction: BlockchainTransaction =
                blockchain_info::blockchain_transaction_request(&tx_id);

            let match_address = String::from(address);

            for tx in &blockchain_transaction.vin {
                if tx.addresses.contains(&match_address) {
                    subtotal_vin += tx.value.parse::<i32>().unwrap();
                }
            }

            for tx in &blockchain_transaction.vout {
                if tx.addresses.contains(&match_address) {
                    subtotal_vout += tx.value.parse::<i32>().unwrap();
                }
            }

            balance += &subtotal_vin - &subtotal_vout;

            println!("----------------------------------------");

            println!("TX ID:        {}", &blockchain_transaction.txid);
            println!("SATOSHI OUT:  {}", &subtotal_vout);
            println!("SATOSHI IN:   {}", &subtotal_vin);
            println!("BALANCE:      {}", &balance);

            println!("----------------------------------------");
        }

        println!("CURRENT BALANCE:  {}", &balance);
        println!("         IN BTC:  {}\n\n", balance as f32 * 0.00000001);
    }
}

fn main() {
    // let entered_address = dotenv::var("WALLET").expect("Error reading env var");

    println!("\n\nWelcome to the Bitcoin wallet analyser!\n\n");
    println!("Please enter your Bitcoin address:");

    let mut entered_address = String::new();

    io::stdin()
        .read_line(&mut entered_address)
        .expect("Error reading input");

    blockchain_info_app(&entered_address);
}
