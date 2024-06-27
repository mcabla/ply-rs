use linked_hash_map::LinkedHashMap;

/// Alias to reduce coupling with `LinkedHashMap`
pub type KeyMap<V> = LinkedHashMap<String, V>;
