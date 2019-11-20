use Food::*;
use rand::prelude::*;

pub struct Dencker {
	max_tolerance: i32,
	current_points: i32,
}

impl Dencker {
	pub fn new() -> Dencker {
		let mut rng = thread_rng();
		Dencker {
			max_tolerance: rng.gen_range(25, 75),
			current_points: 0,
		}
	}
	pub fn feed(&mut self, food: Food) -> bool {
		let mut rng = thread_rng();
		match food {
			Turkey => {
				let add = rng.gen_range(0, 10);
				self.current_points += add;
			},
			MashedPotatos => {
				let add = rng.gen_range(0, 5);
				self.current_points += add;
			},
			GreenBeans => {
				let add = rng.gen_range(2, 15);
				self.current_points += add;
			},
			Casserole => {
				let add = rng.gen_range(10, 20);
				self.current_points += add;
			},
		};
		if self.current_points > self.max_tolerance {
			true
		}
		else {
			false
		}
	}
	pub fn mix(&mut self, foods: &[Food]) -> bool {
		for food in foods {
			match food {
				Turkey => {
					self.current_points += 3;
				},
				MashedPotatos => {
					self.current_points += 1;
				},
				GreenBeans => {
					self.current_points += 5;
				},
				Casserole => {
					self.current_points += 15;
				},
			}
		}
		if self.current_points > self.max_tolerance {
			true
		}
		else {
			false
		}
	}
}

pub enum Food {
	Turkey,
	MashedPotatos,
	GreenBeans,
	Casserole,
}
