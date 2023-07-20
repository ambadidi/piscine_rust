pub fn is_pangram(s: &str) -> bool {
    let alphabet = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let s = s.to_lowercase();
    for char in alphabet {
        if !s.contains(char) { return false;}
    }
    true
}