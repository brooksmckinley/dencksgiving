// Classroom said this assignment was 100 points, so I'm going all out.
// Have fun

extern crate cursive;
extern crate rand;

mod dencker;
mod interface;
mod theme;

use cursive::Cursive;

fn main() {
    let mut siv = Cursive::ncurses().unwrap();
    siv.set_theme(theme::get_theme());
    interface::start_game(&mut siv);
    siv.run();
    println!("Have a great day!");
}
