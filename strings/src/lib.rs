pub fn char_length(s: &str) -> usize {
	let res = s.chars().collect::<Vec<_>>();
	res.len()
}