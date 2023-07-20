pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(),(c as f64).abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut b = String::new();
    let numbers : Vec<f64> = a.split_whitespace().map(|n| n.parse::<f64>().unwrap()).collect();
    for i in numbers {
        b.push_str(i.exp().to_string().as_str());
        b.push(' ');
    }
    b.pop();
    (a, b)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut c : Vec<f64> = Vec::new();
    for i in &b {
        let output = (*i as f64).abs().ln();
        c.push(output);
    }
    (b,c)
}