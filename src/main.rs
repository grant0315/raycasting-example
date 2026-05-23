use raylib::prelude::*;
use std::error::Error;

const SCREEN_HEIGHT: i32 = 640;
const SCREEN_WIDTH: i32 = 480;

const MAP_WIDTH: i32 = 20;
const MAP_HEIGHT: i32 = 20;

const TILE_SIZE: i32 = 16;
const WALL_HEIGHT: i32 = 64;
const CAMERA_PROJECTION_HEIGHT: i32 = 32;

const FOV: f64 = 60.0;

#[derive(Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Self {
            blocked: false,
            block_sight: false,
        }
    }
    pub fn wall() -> Self {
        Self {
            blocked: true,
            block_sight: true,
        }
    }
}

struct Player {
    px: i32,
    py: i32,
    view_angle: f32,
}

struct ProjectionPlane {
    height: i32,
    width: i32,
    center_x: i32,
    center_y: i32,
    distance_from_player: f32,
    angle_between_cols: f32,
}

type Map = Vec<Vec<Tile>>;

fn generate_map() -> Map {
    let mut map: Map = vec![];
    for i in 0..MAP_WIDTH {
        let mut row = Vec::new();
        for j in 0..MAP_HEIGHT {
            let temp: Tile;
            // Determine if border
            if i == 0 || i == MAP_HEIGHT - 1 || j == 0 || j == MAP_WIDTH - 1 {
                temp = Tile::wall();
            } else {
                temp = Tile::empty();
            }
            row.push(temp);
        }
        map.push(row);
    }
    map
}

fn print_map(map: &Map) {
    for (_, row) in map.iter().enumerate() {
        for (_, value) in row.iter().enumerate() {
            match value.blocked {
                true => print!("# "),
                false => print!(". "),
            }
        }
        print!("\n");
    }
}

fn init_wall_texture(
    filepath: &str,
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
) -> Result<Texture2D, raylib::error::Error> {
    let texture = rl.load_texture(&thread, filepath)?;
    texture.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_POINT);
    Ok(texture)
}

fn main() {
    // Init raylib
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_HEIGHT, SCREEN_WIDTH)
        .title("Raycasting example")
        .build();

    let map: Map = generate_map();
    print_map(&map);

    // Load wall texture
    let wall_tex = init_wall_texture(
        "./assets/Torment Textures/True Colour/str_stoneflr5.png",
        &mut rl,
        &thread,
    )
    .expect("Failed load wall texture");

    // Initialize Player
    let player: Player = Player {
        px: 10,
        py: 10,
        view_angle: 45.0,
    };

    // Initialize Projection Plane
    let projection_plane = ProjectionPlane {
        height: SCREEN_HEIGHT,
        width: SCREEN_WIDTH,
        center_x: SCREEN_WIDTH / 2,
        center_y: SCREEN_HEIGHT / 2,
        distance_from_player: (SCREEN_WIDTH as f64 / (FOV.tan() / 2.0)) as f32,
        angle_between_cols: FOV as f32 / SCREEN_WIDTH as f32,
    };

    // --- GAME LOOP ---
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        // Loop through each pixel in SCREEN_WIDTH
        for screen_x in 0..SCREEN_WIDTH {}

        d.clear_background(Color::WHITE);
        d.draw_texture(&wall_tex, 10, 10, Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}

// To render the scene:
// 1. Based on the viewing angle, subtract half of FOV
// 2. Starting from column 0:
//      Cast a ray
//      Trace the ray until it hits a wall
// 3. Record the distance to the wall (the distance is qual to the length of the ray).
// 4. Add the angle increment so that the ray moves to the right (we have this pre-determined in
//    the ProjectPlane object)
// 5. Repeat step 2 and 3 for each subsequent column until all rays are cast
fn raycasting_loop(rl: &RaylibHandle, projection_plane: &ProjectionPlane, player: &Player) {
    // Horizontal or vertical coordinate of intersection
    let mut vertical_grid: i32;
    let mut horizontal_grid: i32;

    let mut dist_to_next_horizontal_grid: i32; // How far the next bound 
    let mut dist_to_next_vertical_grid: i32;
    let mut x_intersection: i32;
    let mut y_intersection: i32;
    let mut dist_to_next_x_intersection: i32;
    let mut dist_to_next_y_intersection: i32;

    let mut x_grid_index: i32; // The current cell the ray is in
    let mut y_grid_index: i32;

    let mut dist_to_vertical_grid_being_hit: i32; // The distance of the x and y ray intersection
    // from the viewport
    let mut dist_to_horizontal_grid_being_hit: i32;

    // FOV is defined, so half would be players direction in the middle
    // We will trace the rays starting from the leftmost ray

    // Ray is between 0 to 180 degress (1st and 2nd quadrant).
    let mut cast_arc: i32 = player.view_angle as i32 / FOV as i32;

    // Wrap around if nessacary
    if cast_arc < 0 {
        cast_arc += 360;
    }

    for screen_x in 0..SCREEN_WIDTH {
        //  Ray is between 0 and 180 degree (1st and 2nd quadrant)

        // Ray is facing down
        if (cast_arc > 0 && cast_arc < 180) {
            // Truncate then add to get the cooridnate of the FIRST grid (horizontal wall) this is
            // in front of the player (this is in pixel unit)
            // ROUNDED DOWN
            horizontal_grid = (player.py / TILE_SIZE) * TILE_SIZE + TILE_SIZE;

            // Compute distance to the next horizontal wall
            dist_to_next_horizontal_grid = TILE_SIZE;

            let mut xtemp = 
        }
    }
}
