use cli::main_loop;
use solana_cloned_validator::runner::{add_accounts_to, create_validator_genesis};

mod cli;

fn main() {
    let mut test_validator = create_validator_genesis(Some(8899));
    add_accounts_to(&mut test_validator);

    let (test_validator, _k) = test_validator.start();

    let rpc_url = test_validator.rpc_url();
    println!("Test validator RPC URL: {}", rpc_url);

    main_loop(&test_validator.get_rpc_client());
}
