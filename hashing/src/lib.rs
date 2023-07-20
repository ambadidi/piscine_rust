use std::collections::HashMap;
pub fn mean(list: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for i in list { sum += i}
    sum as f64 / list.len() as f64
}

pub fn median(list: &Vec<i32>) -> i32 {
    let med;
    let mut sorted: Vec<i32> = Vec::new();
    for i in list {sorted.push(*i)}
    sorted.sort();
    med = if list.len() % 2 == 0 {(sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2}
            else {sorted[sorted.len() / 2]};
    med
}

pub fn mode(list: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for i in list {
        let count = map.entry(*i).or_insert(0);
        *count += 1;
    }
    let mut max_key =0;
    let mut max_value = 0;
    for (key, value) in map {
        if max_value < value {
            max_key = key;
            max_value = value;
        }
    }
    max_key
}