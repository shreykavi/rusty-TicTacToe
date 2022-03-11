use std::io;

fn main() {
    println!("Welcome to TicTacToe!");
    println!("Input the grid size you'd like to play");

    let mut size = String::new();
    io::stdin()
        .read_line(&mut size)
        .expect("No size specified...");
}
