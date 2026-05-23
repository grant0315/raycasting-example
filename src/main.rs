use raylib::prelude::*;
use std::error::Error;

const SCREEN_HEIGHT: i32 = 640;
const SCREEN_WIDTH: i32 = 480;

const MAP_WIDTH: i32 = 20;
const MAP_HEIGHT: i32 = 20;

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

fn cast_ray_get_distance() -> f32;

fn get_wall_intersection_point() -> f32;

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

    // --- GAME LOOP ---
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        for screen_x in 0..SCREEN_WIDTH {
            // Fire ray, calc distance, and get exact intersection coordinates
            let distance = cast_ray_get_distance();
            let wall_hit_x: f32 = get_wall_intersection_point(); // e.g. 5.34

            // Fix fish-eye
            let corrected_distance = distance * player_relative_angle.cos();

            // Calculate screen properties (Destination)
            // Wall height gets smaller the further away it is
            let wall_height = (SCREEN_HEIGHT as f32 / corrected_distance) as f32;
            let screen_y = (SCREEN_HEIGHT as f32 - wall_height) / 2.0; // Center the wall

            // Calculate texture coords
            // Get the fractional part of where the wall was hit (e.g. 0.34)
            let wall_hit_fraction = wall_hit_x - wall_hit_x.floor();

            // Map that fraction to a specific 1-pixel column in the texture file
            let texture_width = wall_tex.width as f32;
            let texture_height = wall_tex.height as f32;
            let tex_x = (wall_hit_fraction * texture_width) as f32;

            // Define raylib rectangles
            let source_rec = Rectangle::new(screen_x as f32, screen_y, 1.0, wall_height);

            // Destination:: draw it at our current horizontal screen column, stretched vertically
            let dest_rec = Rectangle::new(screen_x as f32, screen_y, 1.0, wall_height);

            // Draw the slice
            let origin = Vector2::new(0.0, 0.0);

            // Optional: Add simple distance shadowing by tinting distant walls darker
            let shadow_intensity = (255.0 - (corrected_distance * 10.0)).clamp(50.0, 255.0) as u8;
            let tint = Color::new(shadow_intensity, shadow_intensity, shadow_intensity, 255);

            d.draw_texture_pro(&wall_tex, source_rec, dest_rec, origin, 0.0, tint);
        }

        d.clear_background(Color::WHITE);
        d.draw_texture(&wall_tex, 10, 10, Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
