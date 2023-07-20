use json;
pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
	let mut cals = 0.;
	let mut fats = 0.;
	let mut carbs = 0.;
	let mut prots = 0.;
	for f in foods {
		let cals_text = f.calories[1].clone();
		let cals_val = &cals_text[0..cals_text.len() - 4];
		cals += cals_val.parse::<f64>().unwrap() * f.nbr_of_portions;
		fats += f.fats * f.nbr_of_portions;
		carbs += f.carbs * f.nbr_of_portions;
		prots += f.proteins * f.nbr_of_portions;
	}
	let res = json::object! {
		cals: (cals * 100.0).round() / 100.0,
		carbs: (carbs * 100.0).round() / 100.0,
		proteins: (prots * 100.0).round() / 100.0,
		fats: (fats * 100.0).round() / 100.0
	};
	return res;
}