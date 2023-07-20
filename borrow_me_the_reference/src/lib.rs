pub fn delete_and_backspace(s: &mut String) {
    let cvec: Vec<char> = s.chars().collect();
    let mut nvec :Vec<char> = Vec::new();
    let mut minus = 0;
    let mut plus = 0;
    for ch in cvec {
        if ch != '-' {
            nvec.push(ch);
        }
        else if ch == '-' {
            minus += 1;
        }
        while minus > 0 {
            nvec.pop();
            minus -= 1;
        }
    }
    let mut nvec2 : Vec<char> = Vec::new();
    for ch in nvec.iter().rev() {
        if ch != &'+' {
            nvec2.push(*ch);
        }
        else if ch == &'+' {
            plus += 1;
        }
        while plus > 0 {
            nvec2.pop();
            plus -= 1;
        }
    }
    s.clear();
    s.extend(nvec2.iter().rev());
}

fn transform_vec(v: &Vec<String>) -> Vec<String> {
    let mut res = Vec::new();
    for s in v {
        let mut temp = String::new();
        for c in s.chars() {
            if c == '+' || c == '-' {
                if !temp.is_empty() {
                    res.push(temp);
                    temp = String::new();
                }
                res.push(c.to_string());
            } else {
                temp.push(c);
            }
        }
        if !temp.is_empty() {
            res.push(temp);
        }
    }
    res
}
pub fn do_operations(v: &mut Vec<String>) {
    let splt_vec = transform_vec(v);
    v.clear();
    for (i, ch) in splt_vec.iter().enumerate() {
        if ch == "+" {
            v.push((splt_vec[i-1].parse::<i32>().unwrap() + splt_vec[i+1].parse::<i32>().unwrap()).to_string());
        }
        else if ch == "-" {
            v.push((splt_vec[i-1].parse::<i32>().unwrap() - splt_vec[i+1].parse::<i32>().unwrap()).to_string());
        }
    }
}