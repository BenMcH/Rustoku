use std::iter::FromIterator;

pub type Cell = usize;

pub struct Sudoku {
    board: Vec<Cell>,
    size: usize,
}

impl Sudoku {

    ///Creates a standard sudoku game in a 9x9 grid.
    ///All cells are filled with 0 to start to indicate an empty location.
    pub fn new() -> Sudoku {
        let size = 81usize;
        let board: Vec<Cell> = vec![0; size];
        Sudoku {
            board: board,
            size: 9usize,
        }
    }

    ///Creates a sudoku game based on a set of values that can be iterated over.
    ///Values are read in on a row basis, from left to right, top to bottom.
    ///The first cell read will be in the top left of the game and the last will be in the bottom right
    pub fn from<T: IntoIterator<Item = Cell>>(values: T) -> Sudoku {
        let mut game = Sudoku::new();
        game.set_board(values);
        game
    }

    ///Sets the board to a new set of values.
    ///Values are read in on a row basis, from left to right, top to bottom.
    ///The first cell read will be in the top left of the game and the last will be in the bottom right
    pub fn set_board<T: IntoIterator<Item = Cell>>(&mut self, values: T) {
        let iter = values.into_iter();
        let size = iter.size_hint().1.unwrap_or(0usize);
        let size = (size as f64).sqrt() as usize;
        self.size = size;
        self.board = iter.collect::<Vec<usize>>();
    }

    ///Gets the empty locations on the board and solves the rest of it recursively.
    pub fn solve(&mut self) -> bool {
        let locs = self.get_open_cells_locs();
        self.solve_rec(&locs[..])
    }

    fn solve_rec(&mut self, locs: &[Cell]) -> bool {
        let mut ret = false;
        if !locs.is_empty() {
            let (loc, locs) = locs.split_first()
                .expect("No value even though locs isnt empty...hm");
            let loc = *loc;
            let loc_tuple = (self.get_col(loc), self.get_row(loc));
            self.board[loc] = 0;
            for x in 1..self.size + 1 {
                if self.value_is_safe(loc_tuple, x) {
                    self.board[loc] = x;
                    if self.solve_rec(locs) {
                        ret = true;
                        break;
                    }
                } else {
                    self.board[loc] = 0;
                }
            }
        } else {
            ret = true;
        }
        ret
    }

    fn get_open_cells_locs(&self) -> Vec<Cell> {
        self.board.iter().take(self.size * self.size).enumerate()
            .filter(|&(_, cell)| *cell == 0).map(|(num, _)| num)
            .collect::<Vec<usize>>()
    }

    fn value_is_safe(&self, (col, row): (usize, usize), value: usize) -> bool {
        !(self.row_contains(row, value)   ||
            self.col_contains(col, value) ||
            self.box_contains((col, row), value))
    }

    fn row_contains(&self, row: usize, value: usize) -> bool {
        self.board.iter().skip(row * self.size).take(self.size).any(|cell| *cell == value)
    }

    fn col_contains(&self, col: usize, value: usize) -> bool {
        self.board.iter().enumerate().filter(|&(num, _)| num % self.size == col)
            .any(|(_, cell)| *cell == value)
    }

    fn box_contains(&self, (col, row): (usize, usize), value: usize) -> bool {
        let mut ret = false;
        let size_of_box = (self.size as f64).sqrt() as usize;
        let mut row = (row - row % size_of_box) * self.size;
        let col = col - col % size_of_box;
        //Get row and column of the top left of the box
        for _ in 0..size_of_box {
            ret |= self.board.iter().skip(row + col).take(size_of_box).any(|cell| *cell == value);
            row += self.size;
        }
        ret
    }

    fn get_row(&self, index: Cell) -> usize {
        index / self.size
    }

    fn get_col(&self, index: Cell) -> usize {
        index % self.size
    }

    ///Prints the board out to the screen.
    pub fn print_board(&self) {
        let board = self.get_board();
        for y in 0..self.size {
            let iter = board.iter().skip(y * self.size).take(self.size)
            .fold(String::with_capacity(self.size), |mut acc, &val| {
                let mut ch = format!("{} ", val);
                if val == 0 {
                    ch = format!("_ ");
                }
                acc.push_str(&ch);
                acc
            });
            println!("{}", iter);
        }
    }

    ///Returns the board as a vector of Cells.
    ///The format of this vector is read left to right, top to bottom.
    pub fn get_board(&self) -> Vec<Cell> {
        self.get_board_as::<Vec<Cell>>()
    }

    ///Returns the board contained within a type that implements FromIterator.
    ///The format of this vector is read left to right, top to bottom.
    pub fn get_board_as<T: FromIterator<Cell>>(&self) -> T {
        self.board.iter().map(|cell| *cell).collect::<T>()
    }
}
