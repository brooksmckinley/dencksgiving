pub struct Dencker {
	max_tolerange: i32,
	current_points: i32,
}

impl Dencker {
	pub fn feed(&mut self, food: Food) -> bool {
		unimplemented!()
	}
	pub fn mix(&mut self, foods: &[Food]) -> bool {
		unimplemented!()
	}
}

pub enum Food {
	Turkey,
	MashedPotatos,
	GreenBeans,
	Casserole,
}
