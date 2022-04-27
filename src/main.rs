use bevy::prelude::*;
use regex::Regex;
use std::io;

// Resources:
struct NextPlayer {
    mark: String,
}

impl NextPlayer {
    pub fn switch(&mut self) {
        self.mark = if self.mark == "O".to_string() {
            "X".to_string()
        } else {
            "O".to_string()
        };
    }
}

struct WinSize {
    w: f32,
    h: f32,
}

pub struct Grid {
    grid: Vec<Vec<String>>,
    size: usize,
}

impl Grid {
    pub fn new(m: usize) -> Grid {
        let grid = vec![vec![String::from("_"); m]; m];
        Grid { grid, size: m }
    }

    pub fn set_position(&mut self, mov: &Move, marking: &String) -> bool {
        // Error check the request
        if mov.x >= self.size || mov.y >= self.size || self.grid[mov.y][mov.x] != "_" {
            println!("This is an incorrect move.");
            return false;
        }

        // if not set, set the position
        self.grid[mov.y][mov.x] = (&marking).to_string();
        return true;
    }

    pub fn check_game(&mut self, mov: &Move, marking: &String) -> bool {
        // check vertical, horizontal, and 2 diagonals of x,y

        let answer = vec![marking; self.size];
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
        // print current grid for debugging
        for row in &self.grid {
            println!("{:?}", row);
        }
    }
}

pub struct Move {
    x: usize,
    y: usize,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Rusty Tic Tac Toe".to_string(),
            width: 598.0,
            height: 598.0,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(handle_mouse_clicks)
        .run();
}

const GRID_SPRITE: &str = "grid.png";
const X_SPRITE: &str = "x.png";
const O_SPRITE: &str = "o.png";
const GRID_SIZE: usize = 3; //defaults to grid size 3 // TODO: variable grid size

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>) {
    // Watches for changes in files
    asset_server.watch_for_changes().unwrap();

    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // position window to top left
    let window = windows.get_primary_mut().unwrap();

    // Creates resources that can later be used
    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height(),
    });
    commands.insert_resource(Grid::new(GRID_SIZE));
    commands.insert_resource(NextPlayer {
        mark: "X".to_string(), // starts with X
    });

    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            // x,y,z
            translation: Vec3::new(0., 0., 0.),
            scale: Vec3::new(1., 1., 1.),
            ..Default::default()
        },
        texture: asset_server.load(GRID_SPRITE),
        ..Default::default()
    });
}

fn handle_mouse_clicks(
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    win_size: Res<WinSize>,
    mut grid: ResMut<Grid>,
    mut next_player: ResMut<NextPlayer>,
) {
    let win = windows.get_primary().expect("no primary window");
    let separator_width = win_size.w as usize / GRID_SIZE;

    let mut request = false;
    let mut winner = false;

    if mouse_input.just_pressed(MouseButton::Left) {
        let click_position = win.cursor_position().unwrap();
        let x = click_position[0] as usize / separator_width;
        let y = click_position[1] as usize / separator_width;
        println!("click at {:?}, {:?}", x, y); // debug print

        let parsed_move = Move { x, y };
        request = grid.set_position(&parsed_move, &next_player.mark);
        winner = grid.check_game(&parsed_move, &next_player.mark);
        grid.print(); // debugging print

        if winner {
            // TODO: popup done game
            println!("Player {} has won the game!", next_player.mark);
        }
        if request {
            next_player.switch()
        }
    }
}
