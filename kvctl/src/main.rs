mod cli;

use kv_storage::Store;
use std::collections::HashMap;
use std::io;

fn main() {
    let mut store: Store = HashMap::new();

    println!("Simple KV store");
    println!("Commands: SET key value | GET key | DEL key | EXIT");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        if !cli::handle_command(&mut store, input) {
            break;
        }
    }
}
