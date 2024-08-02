use std::time::Duration;

use solana_client::rpc_client::RpcClient;

pub fn main_loop(rpc_client: &RpcClient) {
    loop {
        let slot = rpc_client.get_slot().unwrap();
        println!("Current slot: [{slot}]");

        std::thread::sleep(Duration::from_secs(1));
    }
}
