mod terminal_interface;

use terminal_interface::{print_grid};

fn main() {
    println!("Hello, world!");

    let mut game_grid = Vec::new();

    game_grid = vec!["0".to_string(); 9];
    print_grid(&game_grid);
}
