pub fn stars(n: u32) -> String {
    let base: usize = 2;
    match n {
        0 => "*".to_string(),
        _ => "*".to_string().repeat(base.pow(n))
    }
}