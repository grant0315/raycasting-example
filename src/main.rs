// src/main.rs
use raylib::math::Vector2;
use raylib::prelude::*;

mod input;
mod map;
mod player;
mod raycasting;
mod renderer;

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 480;
const MAP_WIDTH: i32 = 20;
const MAP_HEIGHT: i32 = 20;
const CELL_SIZE: i32 = 40;
const FOV: f32 = 60.0;

fn main() {
    // Generate complex level layout structure
    let level: map::Map = map::generate_map(MAP_WIDTH, MAP_HEIGHT);
    map::print_map(&level);

    // Dynamic safety spawn finder
    let mut spawn_grid_x = 1;
    let mut spawn_grid_y = 1;

    'finder: for y in 1..(MAP_HEIGHT - 1) {
        for x in 1..(MAP_WIDTH - 1) {
            if level[y as usize][x as usize] == 0 {
                spawn_grid_x = x;
                spawn_grid_y = y;
                break 'finder;
            }
        }
    }

    let initial_pos = Vector2::new(
        (spawn_grid_x as f32 * CELL_SIZE as f32) + (CELL_SIZE as f32 / 2.0),
        (spawn_grid_y as f32 * CELL_SIZE as f32) + (CELL_SIZE as f32 / 2.0),
    );

    let initial_dir = Vector2::new(1.0, 0.0);
    let mut player = player::Player::new(initial_pos, initial_dir, FOV);

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Raycasting Split View Engine")
        .resizable()
        .build();

    // CRITICAL: Make sure your wall_texture.png sits in your root folder!
    let wall_tex = rl
        .load_texture(&thread, "stonebrk1.png")
        .expect("Failed to load wall_texture.png!");

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        let intent = input::poll_player_movement_input(&rl);
        player.update(intent.move_speed, intent.turn_speed, &level, CELL_SIZE, dt);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        let current_width = d.get_screen_width();
        let current_height = d.get_screen_height();

        // 1. Draw 2D Top-Down Schematic View on left
        map::draw_map(&mut d, &level, CELL_SIZE);
        d.draw_circle_v(player.pos, 5.0, Color::RED);

        let look_target = player.pos + (player.dir * 15.0);
        d.draw_line_v(player.pos, look_target, Color::BLUE);

        // 2. Render 3D View plane on right without fisheye errors
        renderer::render_pseudo_3d(
            &mut d,
            &level,
            &wall_tex,
            player.pos,
            player.dir,
            player.fov,
            MAP_WIDTH,
            MAP_HEIGHT,
            CELL_SIZE,
            current_width,
            current_height,
        );
    }
}
