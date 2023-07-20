use std::collections::HashMap;
pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut map1: HashMap<char, i32> = HashMap::new();
    let mut map2: HashMap<char, i32> = HashMap::new();
    for ch in s1.chars() {
        let count = map1.entry(ch).or_insert(0);
        *count += 1;
    }
    for ch in s2.chars() {
        let count = map2.entry(ch).or_insert(0);
        *count += 1;
    }
    if map1 == map2 {return true;}
    false
}