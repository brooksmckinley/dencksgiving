use cursive::Cursive;
use cursive::views::*;

use crate::dencker::Dencker;
use crate::dencker::Food;

pub fn start_game(siv: &mut Cursive) {
	siv.set_user_data(Dencker::new());
	let dialog = Dialog::around(
		LinearLayout::vertical()
			.child(Button::new("Feed Turkey", |s| { button_clicked(s, Food::Turkey) }))
			.child(Button::new("Feed Mashed Potatos", |s| { button_clicked(s, Food::MashedPotatos) }))
			.child(Button::new("Feed Green Beans (with that nasty stuff on top)", |s| { button_clicked(s, Food::GreenBeans) }))
			.child(Button::new("Feed Casserole", |s| { button_clicked(s, Food::Casserole) }))
			.child(Button::new("MIX HIS FOODS", |s| { button_clicked(s, Food::Mix) }))
			.child(DummyView)
			.child(Button::new("I'm done.", finish))
	).title("Feed Mr. Dencker food without making him snap!");
	siv.add_layer(dialog);
}

fn button_clicked(siv: &mut Cursive, food: Food) {
	let dencker: &mut Dencker = siv.user_data().unwrap();
	let res = dencker.feed(food);
	if res {
		fail(siv);
	}
}

fn fail(siv: &mut Cursive) {
	let dialog = Dialog::around(
		LinearLayout::vertical()
			.child(TextView::new("You fed Mr. Dencker too much."))
			.child(Button::new("Quit", |s| { s.quit() }))
	).title("You lose!");
	siv.pop_layer();
	siv.add_layer(dialog);
}

fn finish(siv: &mut Cursive) {
	let dencker: &mut Dencker = siv.user_data().unwrap();
	let score = dencker.get_score();
	let dialog = Dialog::around(
		LinearLayout::vertical()
			.child(TextView::new(format!("You scored {}!", score)))
			.child(Button::new("Quit", |s| { s.quit() }))
	).title("You win!");
	siv.pop_layer();
	siv.add_layer(dialog);
}
