pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut res_vec : Vec<String> = Vec::new();
    for s in names {
        let mut str = String::new();
        for ch in s.chars() {
            if ch.is_uppercase() {
                str.push(ch);
                str.push('.');
                str.push(' ');
            }
        }
        str.pop();
        res_vec.push(str);
    }
    res_vec
}