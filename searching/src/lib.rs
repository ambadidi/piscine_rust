pub fn search(array: &[i32], key: i32) -> Option<usize> {
    for (k, i) in array.into_iter().enumerate() {
        if i == &key {
            return Some(k);
        }
    }
    return None;
}