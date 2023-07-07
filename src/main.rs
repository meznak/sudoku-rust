use std::fmt::{Debug, self};

#[derive(Debug, Default)]
    struct Board {
    cells: [[Cell; 9]; 9],
    solved: usize,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..9 {
            if row > 0 && row % 3 == 0 {
                write!{f, "-----------\n"}?;
            }
            for col in 0..9 {
                if col > 0 && col % 3 == 0 {
                    write!{f, "|"}?;
                }
                write!{f, "{}", self.cells[row][col]}?;
            }
            write!{f, "\n"}?;
        }
        Ok(())
    }
}

impl Board {
    fn new() -> Self {
        Default::default()
    }

    fn initialize(&mut self, input: &str) {
        let mut position = 0;

    for value in input.split(',') {
            let row = position / 9;
            let col = position % 9;
            self.cells[row][col].row = row;
            self.cells[row][col].col = col;
            if value != "-" {
                self.cells[row][col].options = vec![value.parse()
                    .expect("Initialization string must be comma-separated 1-9 or -")];
                self.cells[row][col].solved = true;
                self.solved += 1;
            }
            position += 1;
        }
    }
}

#[derive(Debug, Clone)]
struct Cell {
    row: usize,
    col: usize,
    options: Vec<i8>,
    solved: bool,
}

impl Default for Cell {
    fn default() -> Cell {
            Cell {
                row: 0,
                col: 0,
                options: (1..10)
                    .collect(),
                solved: false,
            }
        }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.solved {
            write!{f, "{}", self.options[0]}
        } else {
            write!{f, " "}
        }
    }
}

impl Board {
    fn eliminate_conflicts(&mut self) {
        let cells = self.cells.clone();

        self.cells.iter_mut()
            .flat_map(|c| c.iter_mut())
            .for_each(|c| {
                if !c.solved {
                    if c.eliminate_conflicts(&cells) {
                        self.solved += 1
                    }
                }
            });
    }
}

impl Cell {
    fn eliminate_conflicts(&mut self, cells: &[[Cell; 9]; 9]) -> bool {
        // row
        for cell in &cells[self.row] {
            if self.check_remove_option(&cell) {
                return true;
            }
        }

        // col
        for cell in &cells[0..9][self.col] {
            if self.check_remove_option(&cell) {
                return true;
            }
        }

        // square
        let first_row = self.row / 3 * 3;
        let last_row = first_row + 2;
        let first_col = self.col / 3 * 3;
        let last_col = first_col + 2;

        for row in &cells[first_row..last_row] {
            for cell in &row[first_col..last_col] {
                if self.check_remove_option(&cell) {
                    return true;
                }
            }
        }

        false
    }

    fn check_remove_option(&mut self, other: &Cell) -> bool {
            if !(other.row == self.row && other.col == self.col) {
                if other.solved {
                    self.options.retain(|&x| x != other.options[0]);
                }

                if self.options.len() == 1 {
                    self.solved = true;
                }
            }

            self.solved
    }
}

fn main() {
    // let sample = "1,2,3,4,5,6,7,8,-,2,-,-,-,-,-,-,-,-,3,-,-,-,-,-,-,-,-,4,-,-,-,-,-,-,-,-,5,-,-,-,-,-,-,-,-,6,-,-,-,-,-,-,-,-,7,-,-,-,-,-,-,-,-,8,-,-,-,-,-,-,-,-,-,-,-,-,-,-,-,-,-";
    let sample = "6,-,5,-,-,-,-,2,-,-,-,7,9,6,4,-,-,5,-,9,1,-,2,-,-,-,-,-,8,-,5,9,6,-,-,3,-,3,-,8,-,1,-,5,-,4,-,-,3,7,2,-,6,-,-,-,-,-,8,-,5,9,-,5,-,-,6,1,9,8,-,-,-,1,-,-,-,-,7,-,6";
    // let sample: &str = "5,3,-,-,7,-,-,-,-,6,-,-,1,9,5,-,-,-,-,9,8,-,-,-,6,-,8,-,-,-,6,-,-,-,3,4,-,-,8,-,3,-,-,1,7,-,-,-,2,-,-,-,6,-,6,-,-,-,2,8,-,-,-,4,1,9,-,-,5,-,-,-,8,-,-,7,9";
    let mut board = Board::new();
    board.initialize(&sample);

    print!("loaded: {}\n{}", board.solved, board);

    while board.solved < 81 {
        board.eliminate_conflicts();
        print!("\nSolved: {}\n{}\n", board.solved, board);
    }
}
