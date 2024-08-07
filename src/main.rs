use cli::main_loop;
use solana_cloned_validator::runner::{add_accounts_to, create_validator_genesis};
use std::env;

mod cli;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Provide path where JSON files with accounts data are stored as first argument!");
        return;
    }

    let mut test_validator = create_validator_genesis(Some(8899));

    match add_accounts_to(&mut test_validator, &args[1]) {
        Ok(_) => println!("Adding account finished. All files processed."),
        Err(err) => eprintln!("Couldn't add all files. Reason: {err:?}"),
    }

    let (test_validator, _k) = test_validator.start();

    let rpc_url = test_validator.rpc_url();
    println!("Test validator RPC URL: {}", rpc_url);

    main_loop(&test_validator.get_rpc_client());
}
