use std::collections::HashMap;

type Store = HashMap<String, String>;

fn set(store: &mut Store, key: String, value: String) -> Option<String> {
    // вставь в HashMap и верни старое значение, если было
    store.insert(key, value)
}

fn get<'a>(store: &'a Store, key: &str) -> Option<&'a String> {
    // верни ссылку на значение или None
    store.get(key)
}

fn delete(store: &mut Store, key: &str) -> Option<String> {
    // удали по ключу и верни прежнее значение
    store.remove(key)
}

fn main() {
    let mut store: Store = HashMap::new();

    set(&mut store, "name".to_string(), "Alice".to_string());
    set(&mut store, "lang".to_string(), "Rust".to_string());

    if let Some(name) = get(&store, "name") {
        println!("name = {name}");
    }

    let old = set(&mut store, "name".to_string(), "Bob".to_string());
    println!("old name = {old:?}");

    let removed = delete(&mut store, "lang");
    println!("removed lang = {removed:?}");

    println!("store = {store:?}");
}
