pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
	let s_split = s.split_ascii_whitespace();
	let mut res: Vec<u32> = Vec::new();
	for str in s_split {
		if str.ends_with("k") {
			let new_str = str.replace("k", "");
			let nbr = new_str.parse::<f32>().expect("wrong format for a number");
			res.push((nbr * 1000.) as u32);
		} else {
			let nbr = str.parse::<f32>().expect("wrong format for a number");
			res.push(nbr as u32)
		}
	}
	Box::new(res)
}
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
	*a
}