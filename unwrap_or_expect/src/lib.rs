pub enum Security {
	Unknown,
	High,
	Medium,
	Low,
	BlockServer,
}

pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {
    match security_level {
		Security::Unknown => server.unwrap(),
		Security::High => server.expect("ERROR: program stops"),
		Security::Medium => server.unwrap_or("WARNING: check the server".to_owned()),
		Security::Low => server.unwrap_or_else(msg),
		Security::BlockServer => server.unwrap_err()
	}
}
pub fn msg(url: String) -> String {
	let mut res = String::new();
	res.push_str("Not found: ");
	res.push_str(&url);
	res
}