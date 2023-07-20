pub fn first_fifty_even_square() -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
    let mut c = || {
        for x in 2..101 {
            if x % 2 == 0 {
                vec.push(x * x)
            }
        }
    };
    c();
    vec
}
