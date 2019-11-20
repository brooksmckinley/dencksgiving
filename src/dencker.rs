use Food::*;
use rand::prelude::*;

pub struct Dencker {
	max_tolerance: i32,
	current_points: i32,
	current_score: i32,
}

impl Dencker {
	pub fn new() -> Dencker {
		let mut rng = thread_rng();
		Dencker {
			max_tolerance: rng.gen_range(25, 75),
			current_points: 0,
			current_score: 0,
		}
	}
	pub fn feed(&mut self, food: Food) -> bool {
		let mut rng = thread_rng();
		match food {
			Turkey => {
				self.current_points += rng.gen_range(0, 10);
				self.current_score += 3;
			},
			MashedPotatos => {
				self.current_points += rng.gen_range(0, 5);
				self.current_score += 1;
			},
			GreenBeans => {
				self.current_points += rng.gen_range(2, 15);
				self.current_score += 5;
			},
			Casserole => {
				self.current_points += rng.gen_range(10, 20);
				self.current_score += 15;
			},
			Mix => {
				self.current_points += rng.gen_range(10, 25);
				self.current_score += 30;
			}
		};
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
	Mix,
}
