use std::io;

pub struct Grid {
    grid: Vec<Vec<String>>,
}

impl Grid {
    pub fn new(m: usize) -> Grid {
        let mut grid = vec![vec![String::from("_"); m]; m];
        Grid { grid }
    }

    pub fn set_position(&mut self, x: usize, y: usize, marking: String) {
        // if not set, set the position
        self.grid[y][x] = marking;
    }

    pub fn print(&self) {
        // if not set, set the position
        for row in &self.grid {
            println!("{:?}", row);
        }
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

    // players 1 and -2
    let mut next_player = "X";

    // Loop turn by turn to get next move
    // Stop loop when either wins or theres a draw
    loop {
        next_player = if next_player == "O" { "X" } else { "O" };
        println!("Player {}'s turn", next_player);
        println!("Input the next move");

        let mut next_move = String::new();
        io::stdin()
            .read_line(&mut next_move)
            .expect("No move specified!");

        grid.set_position(0, 0, String::from(next_player));
        grid.print();
    }
}
