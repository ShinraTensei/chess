mod main_window;

use main_window::Game;

fn main() {
    let mut g = Game::new().unwrap();
    let _ = g.start();
}
