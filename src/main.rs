use regex::Regex;
use std::io;

pub struct Grid {
    grid: Vec<Vec<String>>,
    size: usize,
}

impl Grid {
    pub fn new(m: usize) -> Grid {
        let grid = vec![vec![String::from("_"); m]; m];
        Grid { grid, size: m }
    }

    pub fn set_position(&mut self, mov: &Move, marking: String) -> bool {
        // TODO: error check the request
        if mov.x >= self.size || mov.y >= self.size || self.grid[mov.y][mov.x] != "_" {
            println!("This is an incorrect move.");
            return false;
        }

        // if not set, set the position
        self.grid[mov.y][mov.x] = marking;
        return true;
    }

    pub fn check_game(&mut self, mov: &Move, marking: String) -> bool {
        // check vertical, horizontal, and 2 diagonals of x,y

        let answer = vec![&marking; self.size];
        let mut horizontal: Vec<&String> = Vec::new();
        let mut vertical: Vec<&String> = Vec::new();
        let mut diagonal1: Vec<&String> = Vec::new();
        let mut diagonal2: Vec<&String> = Vec::new();

        for coordinate in 0..self.size {
            horizontal.push(&self.grid[mov.y][coordinate]);
            vertical.push(&self.grid[coordinate][mov.x]);
            diagonal1.push(&self.grid[coordinate][coordinate]);
            diagonal2.push(&self.grid[coordinate][self.size - 1 - coordinate]);
        }

        if vertical == answer || horizontal == answer || diagonal1 == answer || diagonal2 == answer
        {
            return true;
        }
        return false;
    }

    pub fn print(&self) {
        // if not set, set the position
        for row in &self.grid {
            println!("{:?}", row);
        }
    }
}

pub struct Move {
    x: usize,
    y: usize,
}

impl Move {
    pub fn new(next_move: String) -> Move {
        // Regex for grid locations
        let re = Regex::new(r"(\d{1}),(\d{1})").unwrap();

        let location = next_move.trim();
        let cap = re.captures(location).expect("Need a move of type: x,y");

        let x = cap[1].parse::<usize>().unwrap();
        let y = cap[2].parse::<usize>().unwrap();
        println!("x: {} y: {}", x, y);
        Move { x, y }
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

    // players X and O
    let mut next_player = "X";

    let mut winner = false;

    // Loop turn by turn to get next move
    // Stop loop when either wins or theres a draw
    while !winner {
        next_player = if next_player == "O" { "X" } else { "O" };
        println!("Player {}'s turn", next_player);
        println!("Input the next move (in format 'x,y')");

        let mut request = false;
        // Loop until valid move supplied
        while !request {
            let mut next_move = String::new();
            io::stdin()
                .read_line(&mut next_move)
                .expect("No move specified!");

            let parsed_move = Move::new(next_move);
            request = grid.set_position(&parsed_move, String::from(next_player));
            winner = grid.check_game(&parsed_move, String::from(next_player));
        }
        grid.print();
    }
    println!("Player {} has won the game!", next_player);
}
