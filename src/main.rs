use std::fmt::{Debug, Formatter, self};

#[derive(Debug, Default)]
    struct Board {
    cells: [[Cell; 9]; 9],
    solved: bool,
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
            if value != "-" {
                let row = position / 9;
                let col = position % 9;
                self.cells[row][col].options = vec![value.parse()
                    .expect("Initialization string must be comma-separated 1-9 or -")];
                self.cells[row][col].solved = true;
            }
            position += 1;
        }
    }
}

#[derive(Debug)]
struct Cell {
    options: Vec<i8>,
    solved: bool,
}

impl Default for Cell {
fn default() -> Cell {
        Cell {
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

impl Cell {
    fn get_value(&self) -> Option<i8> {
        if self.solved {
            Some(self.options[0])
        } else {
            None
        }
    }
}

fn main() {
    let sample: &str = "5,3,-,-,7,-,-,-,-,6,-,-,1,9,5,-,-,-,-,9,8,-,-,-,6,-,8,-,-,-,6,-,-,-,3,4,-,-,8,-,3,-,-,1,7,-,-,-,2,-,-,-,6,-,6,-,-,-,2,8,-,-,-,4,1,9,-,-,5,-,-,-,8,-,-,7,9";
    let mut c = Board::new();
    c.initialize(&sample);

        // [
        //     [5,3,-1,-1,7,-1,-1,-1,-1],
        //     [6,-1,-1,1,9,5,-1,-1,-1],
        //     [-1,9,8,-1,-1,-1,-1,6,-1],
        //     [8,-1,-1,-1,6,-1,-1,-1,3],
        //     [4,-1-1,8,-1,3,-1,-1,1],
        //     [7,-1,-1,-1,2,-1,-1,-1,6],
        //     [-1,6,-1-1,-1,2,8,-1],
        //     [-1,-1,4,1,9,-1,-1,5],
        //     [-1,-1,-1,8,-1,-1,7,9]
        // ]

    print!("loaded:\n{}", c);
}
