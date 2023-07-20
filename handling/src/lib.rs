use std::fs::{self,File};
use std::io::ErrorKind;

pub fn open_or_create(file: &str, content: &str) {
	let open_result = File::open(file);
	match open_result {
		Ok(_opened) => fs::write(file, content).unwrap(),
		Err(error) => match error.kind() {
			ErrorKind::NotFound => match File::create(file) {
				Ok(_created) => fs::write(file, content).unwrap(),
				Err(err) => panic!("{}", err)
			},
			other_error => panic!("{}", other_error)
		}
	}
}