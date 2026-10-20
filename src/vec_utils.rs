pub fn has(flag: &str, vec: &[String]) -> bool {
    vec.iter().any(|s| s == flag)
} 