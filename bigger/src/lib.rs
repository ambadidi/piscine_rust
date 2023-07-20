use std::collections::HashMap;
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut max_value = 0;
    for (_, value) in h {
        if max_value < value { max_value = value;}
    }
    max_value
}