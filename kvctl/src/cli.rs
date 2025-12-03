use kv_storage::{delete, get, set, Store};

pub fn handle_command(store: &mut Store, input: &str) -> bool {
    let parts: Vec<&str> = input.split_whitespace().collect();

    match parts.as_slice() {
        ["SET", key, value] => match set(store, (*key).to_string(), (*value).to_string()) {
            Some(old_val) => println!("OK (replaced old value: {old_val})"),
            None => println!("OK (new key)"),
        },
        ["GET", key] => {
            if let Some(val) = get(store, key) {
                println!("value for key {key} is {val}");
            } else {
                println!("value not found for {key}");
            }
        }
        ["DEL", key] => match delete(store, key) {
            Some(old_val) => println!("deleted value for key {key}: {old_val}"),
            None => println!("no value found for {key}"),
        },
        ["EXIT"] => {
            println!("Bye!");
            return false;
        }
        _ => println!("Unknown or invalid command"),
    }

    true
}
