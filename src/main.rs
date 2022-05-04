use bevy::{prelude::*, winit::WinitSettings};

const GRID_SPRITE: &str = "grid.png";
const X_SPRITE: &str = "x.png";
const O_SPRITE: &str = "o.png";
const GRID_SIZE: usize = 3; //defaults to grid size 3 // TODO: variable grid size
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

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
    pub fn sprite(&mut self) -> &str {
        if self.mark == "X".to_string() {
            return X_SPRITE;
        } else {
            return O_SPRITE;
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
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(handle_mouse_clicks)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>) {
    // Watches for changes in files
    asset_server.watch_for_changes().unwrap();

    // cameras (UI and 2D for game)
    commands.spawn_bundle(UiCameraBundle::default());
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
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    asset_server: Res<AssetServer>,
    win_size: Res<WinSize>,
    mut grid: ResMut<Grid>,
    mut next_player: ResMut<NextPlayer>,
) {
    let win = windows.get_primary().expect("no primary window");
    let separator_width = win_size.w as usize / GRID_SIZE;

    let mut request = false;
    let mut winner = false;

    // For testing
    if mouse_input.just_pressed(MouseButton::Right) {
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                color: Color::NONE.into(),
                ..default()
            })
            .with_children(|parent| {
                // absolute positioning
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Px(400.0), Val::Px(300.0)),
                            position_type: PositionType::Absolute,
                            position: Rect {
                                left: Val::Px(win_size.w as f32 / 2. - 200.),
                                bottom: Val::Px(win_size.h as f32 / 2. - 150.),
                                ..default()
                            },
                            border: Rect::all(Val::Px(10.0)),
                            ..default()
                        },
                        color: Color::rgb(0.4, 0.4, 1.0).into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                    flex_direction: FlexDirection::ColumnReverse,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    align_content: AlignContent::Center,
                                    ..default()
                                },
                                color: Color::rgb(0.8, 0.8, 1.0).into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn_bundle(TextBundle {
                                    style: Style {
                                        // center button
                                        margin: Rect::all(Val::Auto),
                                        // horizontally center child text
                                        justify_content: JustifyContent::Center,
                                        // vertically center child text
                                        align_items: AlignItems::Center,
                                        align_content: AlignContent::Center,
                                        max_size: Size::new(Val::Px(350.0), Val::Px(100.0)),
                                        ..default()
                                    },
                                    text: Text::with_section(
                                        "Player X wins!",
                                        TextStyle {
                                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                            font_size: 40.0,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                        TextAlignment {
                                            vertical: VerticalAlign::Center,
                                            horizontal: HorizontalAlign::Center,
                                        },
                                        // Default::default(),
                                    ),
                                    ..default()
                                });
                                parent
                                    .spawn_bundle(ButtonBundle {
                                        style: Style {
                                            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                            // center button
                                            margin: Rect::all(Val::Auto),
                                            // horizontally center child text
                                            justify_content: JustifyContent::Center,
                                            // vertically center child text
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        color: NORMAL_BUTTON.into(),
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn_bundle(TextBundle {
                                            text: Text::with_section(
                                                "Restart",
                                                TextStyle {
                                                    font: asset_server
                                                        .load("fonts/FiraSans-Bold.ttf"),
                                                    font_size: 40.0,
                                                    color: Color::rgb(0.9, 0.9, 0.9),
                                                },
                                                Default::default(),
                                            ),
                                            ..default()
                                        });
                                    });
                            });
                    });
            });
    }

    if mouse_input.just_pressed(MouseButton::Left) {
        let click_position = win.cursor_position().unwrap();
        let x = click_position[0] as usize / separator_width;
        let y = click_position[1] as usize / separator_width;

        let parsed_move = Move { x, y };
        request = grid.set_position(&parsed_move, &next_player.mark);
        winner = grid.check_game(&parsed_move, &next_player.mark);

        if winner {
            // TODO: popup done game

            println!("Player {} has won the game!", next_player.mark);
        }
        if request {
            // Draw mark
            let sprite = commands.spawn_bundle(SpriteBundle {
                transform: Transform {
                    // x,y,z
                    translation: Vec3::new(
                        // - GRID_SIZE/2 on x and y to offset axis since Transforms have 0,0 at center
                        (separator_width as f32 * (x as f32 - (GRID_SIZE / 2) as f32)),
                        (separator_width as f32 * (y as f32 - (GRID_SIZE / 2) as f32)),
                        1.,
                    ),
                    scale: Vec3::new(0.25, 0.25, 1.),
                    ..Default::default()
                },
                texture: asset_server.load(next_player.sprite()),
                ..Default::default()
            });

            next_player.switch();
        }
    }
}
