
pub struct Sudoku {
    board: [[i32; 9]; 9],
}

impl Sudoku {
    pub fn new(board: [[i32; 9]; 9]) -> Sudoku {
        Sudoku { board }
    }

    pub fn solve(&mut self) -> bool {
        let empty = self.find_empty();
        match empty {
            None => true,
            Some((row, col)) => {
                for num in 1..=9 {
                    if self.is_valid(row, col, num) {
                        self.board[row][col] = num;
                        if self.solve() {
                            return true;
                        }
                        self.board[row][col] = 0;
                    }
                }
                false
            }
        }
    }

    fn find_empty(&self) -> Option<(usize, usize)> {
        for i in 0..9 {
            for j in 0..9 {
                if self.board[i][j] == 0 {
                    return Some((i, j));
                }
            }
        }
        None
    }

    fn is_valid(&self, row: usize, col: usize, num: i32) -> bool {
        // Check row
        for j in 0..9 {
            if self.board[row][j] == num {
                return false;
            }
        }

        // Check column
        for i in 0..9 {
            if self.board[i][col] == num {
                return false;
            }
        }

        // Check 3x3 box
        let box_row = row - row % 3;
        let box_col = col - col % 3;
        for i in 0..3 {
            for j in 0..3 {
                if self.board[box_row + i][box_col + j] == num {
                    return false;
                }
            }
        }

        true
    }

    pub fn print(&self) {
        for i in 0..9 {
            if i % 3 == 0 && i != 0 {
                println!("-----------------------");
            }
            for j in 0..9 {
                if j % 3 == 0 && j != 0 {
                    print!("| ");
                }
                print!("{} ", self.board[i][j]);
            }
            println!();
        }
    }
}
