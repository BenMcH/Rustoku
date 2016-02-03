extern crate rustoku;
use rustoku::Sudoku;

fn main() {
    let board = vec![4,2,0,0,5,0,0,0,8,
                     8,0,3,0,0,0,0,0,0,
                     0,1,0,0,0,9,0,0,0,
                     0,7,0,0,8,0,6,0,0,
                     0,0,0,5,2,6,0,0,0,
                     0,0,5,0,3,0,0,9,0,
                     0,0,0,2,0,0,0,7,0,
                     0,0,0,0,0,0,5,0,3,
                     9,0,0,0,4,0,0,2,6];
    let mut game = Sudoku::from(board);
    println!("Before:");
    game.print_board();
    println!("");
    game.solve();
    println!("After:");
    game.print_board();
}
