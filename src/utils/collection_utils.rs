use std::collections::HashSet;

pub fn join_hash_set_of_strings(hash_set: &HashSet<String>, join: &str) -> String {
    hash_set
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<_>>()
        .join(join)
}
