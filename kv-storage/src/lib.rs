use std::collections::HashMap;

pub type Store = HashMap<String, String>;

pub fn set(store: &mut Store, key: String, value: String) -> Option<String> {
    // вставь в HashMap и верни старое значение, если было
    store.insert(key, value)
}

pub fn get<'a>(store: &'a Store, key: &str) -> Option<&'a String> {
    // верни ссылку на значение или None
    store.get(key)
}

pub fn delete(store: &mut Store, key: &str) -> Option<String> {
    // удали по ключу и верни прежнее значение
    store.remove(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_inserts_new_key() {
        let mut store: Store = HashMap::new();

        let old = set(&mut store, "name".to_string(), "Alice".to_string());

        assert!(old.is_none());
        assert_eq!(get(&store, "name"), Some(&"Alice".to_string()));
    }

    #[test]
    fn set_replaces_existing_value() {
        let mut store: Store = HashMap::new();

        set(&mut store, "name".to_string(), "Alice".to_string());
        let old = set(&mut store, "name".to_string(), "Bob".to_string());

        assert_eq!(old, Some("Alice".to_string()));
        assert_eq!(get(&store, "name"), Some(&"Bob".to_string()));
    }

    #[test]
    fn get_returns_none_for_missing_key() {
        let store: Store = HashMap::new();

        assert_eq!(get(&store, "missing"), None);
    }

    #[test]
    fn delete_removes_existing_key() {
        let mut store: Store = HashMap::new();

        set(&mut store, "lang".to_string(), "Rust".to_string());

        let removed = delete(&mut store, "lang");
        assert_eq!(removed, Some("Rust".to_string()));
        assert_eq!(get(&store, "lang"), None);
    }

    #[test]
    fn delete_on_missing_key_returns_none() {
        let mut store: Store = HashMap::new();

        let removed = delete(&mut store, "nope");
        assert!(removed.is_none());
    }
}
