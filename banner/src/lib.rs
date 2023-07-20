use std::collections::HashMap;
use std::num::ParseFloatError;
pub struct Flag {
    // expected public fields
	pub short_hand: String,
	pub long_hand: String,
	pub desc: String
}

impl Flag {
    pub fn opt_flag(l_h: &str, d: &str) -> Flag {
		let mut s_h = String::new();
		s_h.push_str("-");
		s_h.push_str(&l_h[0..1]);
		let mut long = String::new();
		long.push_str("--");
		long.push_str(&l_h);
		Flag { short_hand: s_h, long_hand: long, desc: d.to_owned() }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {
		self.flags.insert(flag, func);
    }
    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {
		match self.flags[&flag](argv[0], argv[1]) {
			Ok(res) => res,
			Err(_) => "invalid float literal".to_owned()
		}
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
	let num1 = a.parse::<f32>()?;
	let num2 = b.parse::<f32>()?;
	let res = num1/num2;
	Ok(res.to_string())
}
pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
	let num1 = a.parse::<f32>()?;
	let num2 = b.parse::<f32>()?;
	let res = num1%num2;
	Ok(res.to_string())
}
