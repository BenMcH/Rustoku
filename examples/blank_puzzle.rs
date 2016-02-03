extern crate rustoku;
use rustoku::Sudoku;

fn main() {
    let mut game = Sudoku::new();
    println!("Starting:");
    game.print_board();
    println!("");
    game.solve();
    println!("Solved:");
    game.print_board();
}
