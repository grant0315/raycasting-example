use raylib::prelude::*;

mod input;
mod map;
mod player;
mod raycasting;
mod renderer;

const SCREEN_HEIGHT: i32 = 640;
const SCREEN_WIDTH: i32 = 480;

const MAP_WIDTH: i32 = 20;
const MAP_HEIGHT: i32 = 20;

const CELL_SIZE: i32 = 20;

const FOV: f32 = 60.0;

fn main() {
    // Init level
    let level: map::Map = map::generate_map(MAP_WIDTH, MAP_HEIGHT);
    map::print_map(&level);

    // Init player
    let initial_pos = Vector2::new(10.0, 10.0);
    let initial_dir = Vector2::new(-1.0, 0.0);
    let velocity = Vector2::new(0.0, 0.0);

    let mut player = player::Player::new(initial_pos, initial_dir, velocity, FOV);

    // Init raylib
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_HEIGHT, SCREEN_WIDTH)
        .title("Raycasting 2D")
        .build();

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        let player_velocity = input::poll_player_movement_input(&rl);
        player.velocity = player_velocity;
        player.update(dt);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        // Draw out current state of map
        map::draw_map(&mut d, &level, CELL_SIZE);
    }
}
