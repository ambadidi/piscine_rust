pub fn capitalize_first(input: &str) -> String {
    if input.is_empty() { return "".to_owned();}
    let output = input[0..1].to_uppercase() + &input[1..];
    output
}

pub fn title_case(input: &str) -> String {
    let vec = input.split_whitespace()
            .map(|s| capitalize_first(s)).collect::<Vec<_>>();
    vec.join(" ")
}

pub fn change_case(input: &str) -> String {
    let vec = input.chars().collect::<Vec<_>>();
    let mut output = String::new();
    for i in vec {
        if i.is_lowercase() {
            output.push(i.to_uppercase().next().unwrap());
        } else if i.is_uppercase() {
            output.push(i.to_lowercase().next().unwrap());
        } else { output.push(i);}
    }
    output
}