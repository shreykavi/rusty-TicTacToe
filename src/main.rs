use std::io;

pub struct Grid {
    grid: Vec<Vec<i32>>,
}

impl Grid {
    pub fn new(m: usize) -> Grid {
        let mut grid = vec![vec![0; m]; m];
        Grid { grid }
    }

    pub fn set_position(&mut self, x: usize, y: usize, marking: i32) {
        // if not set, set the position
        self.grid[y][x] = marking;
    }

    pub fn print(&self) {
        // if not set, set the position
        println!("{:?}", self.grid);
    }
}

fn main() {
    println!("Welcome to TicTacToe!");
    println!("Input the grid size you'd like to play: ");

    let mut size = String::new();
    io::stdin()
        .read_line(&mut size)
        .expect("No size specified!");

    // Create grid
    let mut grid = Grid::new(size.trim().parse().expect("Expected an int!"));

    // Loop turn by turn to get next move
    // Stop loop when either wins or theres a draw
    loop {
        println!("Input the next move");

        let mut next_move = String::new();
        io::stdin()
            .read_line(&mut next_move)
            .expect("No move specified!");

        grid.set_position(0, 0, 1);
        grid.print();
    }
}
