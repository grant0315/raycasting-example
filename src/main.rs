use raylib::prelude::*;

mod input;
mod map;
mod player;

const SCREEN_HEIGHT: i32 = 640;
const SCREEN_WIDTH: i32 = 480;

const MAP_WIDTH: i32 = 20;
const MAP_HEIGHT: i32 = 20;

const CELL_SIZE: i32 = 20;

const FOV: f32 = 60.0;

fn main() {
    // Init level
    let mut level: map::Map = map::generate_map(MAP_WIDTH, MAP_HEIGHT);
    map::print_map(&level);

    // Init player
    let initial_pos = Vector2::new(10.0, 10.0);
    let initial_dir = Vector2::new(-1.0, 0.0);
    let velocity = Vector2::new(0.0, 0.0);

    let mut player = player::Player::new(initial_pos, initial_dir, velocity);

    // Init raylib
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_HEIGHT, SCREEN_WIDTH)
        .title("Raycasting 2D")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        // Poll for player movement and update
        input::poll_player_movement_input(&rl, player.velocity);

        // Draw out current state of map
        map::draw_map(&mut d, &level, CELL_SIZE);
    }
}
