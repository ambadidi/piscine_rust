#[allow(unused_mut)]
pub fn first_subword(mut s: String) -> String {
    let mut subword = String::new();
    let mut capi = true;
    for ch in s.chars() {
        if ch == '_' || (ch.is_uppercase() && !capi) {
            break;
        }
        subword.push(ch);
        capi = false;
    }
    subword
}