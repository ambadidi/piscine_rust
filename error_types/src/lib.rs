pub use chrono::{Utc, NaiveDate};

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    // expected public fields
    pub form_values: (String, String),
    pub date: String,
    pub err: String
}
impl FormError {
    pub fn new(field_name: String, field_value: String, err: String) -> FormError {
        let date = Utc::now();
        let formatted = format!("{}", date.format("%Y-%m-%d %H:%M:%S"));
        FormError { form_values: (field_name, field_value), date: formatted, err: err }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    // expected public fields
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub birth_location: String,
    pub password: String,
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        birth_location: String,
        password: String,
    ) -> Form {
        Form { first_name, last_name, birth, birth_location, password }
    }

    pub fn validate(&self) -> Result<Vec<&str>, FormError> {
        let name = self.first_name.clone();
        if name.is_empty() {
            return Err(FormError::new("first_name".to_owned(), name, "No user name".to_owned()));
        }
        let password = self.password.clone();
        if password.len() < 8 {
            return Err(FormError::new("password".to_owned(), password, "At least 8 characters".to_owned()));
        }
        let mut num = false;
        let mut alpha = false;
        let mut symb = false;
        for ch in password.chars() {
            if ch.is_numeric() { num = true}
            else if ch.is_alphabetic() { alpha = true}
            else { symb = true}
        }
        if !num || !alpha || !symb {
            return Err(FormError::new("password".to_owned(), password, "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)".to_owned()));
        }
        let mut valid: Vec<&str> = Vec::new();
        valid.push("Valid first name");
        valid.push("Valid password");
        Ok(valid)
    }
}

// pub fn create_date(date: &str) -> NaiveDate {
//     match NaiveDate::parse_from_str(&date, "%Y-%m-%d"){
//         Ok(d) => d,
//         Err(err) => NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d").unwrap(),
//     }
// }