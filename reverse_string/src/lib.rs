pub fn rev_str(input: &str) -> String {
    let mut output = String::new();
    for ch in input.chars().rev() {
        output.push(ch);
    }
    output
}