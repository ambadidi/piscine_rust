pub fn arrange_phrase(phrase: &str) -> String {
    let list : Vec<&str> = phrase.split(" ").collect();
    let mut vec : Vec<usize> = Vec::new();
    for i in 1..list.len() + 1 {
        let index = i.to_string();
        vec.push(list.iter().position(|&p| p.contains(&index)).unwrap());
    }
    let mut res_vec : Vec<String> = Vec::new();
    for i in vec.iter() {
        let res = list[*i].replace(['1','2','3','4','5','6','7','8','9'], "");
        res_vec.push(res);
    }
    res_vec.join(" ")
}