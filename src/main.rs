// Classroom said this assignment was 100 points, so I'm going all out.
// Have fun

extern crate cursive;
extern crate rand;

mod dencker;
mod interface;

use cursive::Cursive;

fn main() {
    let mut siv = Cursive::ncurses().unwrap();
    interface::start_game(&mut siv);
    siv.run();
    println!("Have a great day!");
}
