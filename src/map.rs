// src/map.rs
use raylib::prelude::*;

pub type Map = Vec<Vec<i8>>;

struct RectRoom {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl RectRoom {
    fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    fn center(&self) -> (i32, i32) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }
}

// A simple local PRNG to guarantee compilation without external crates
struct SimpleRng {
    state: u32,
}

impl SimpleRng {
    fn new(seed: u32) -> Self {
        Self { state: seed }
    }

    // Returns a pseudo-random value between min and max (inclusive)
    fn gen_range(&mut self, min: i32, max: i32) -> i32 {
        // Classic LCG multiplier parameters
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        let random_u32 = (self.state / 65536) % 32768;

        let range = (max - min) + 1;
        min + (random_u32 as i32 % range)
    }
}

pub fn generate_map(m_width: i32, m_height: i32) -> Map {
    // 1. Initialize the map as one solid wall block (1)
    let mut map: Map = vec![vec![1; m_width as usize]; m_height as usize];

    // Seed the local generator with a variable number
    let mut rng = SimpleRng::new(1337);

    let max_rooms = 6;
    let min_room_size = 4;
    let max_room_size = 7;

    let mut placed_rooms: Vec<RectRoom> = Vec::new();

    // 2. Attempt to carve out random rooms
    for _ in 0..max_rooms {
        let w = rng.gen_range(min_room_size, max_room_size);
        let h = rng.gen_range(min_room_size, max_room_size);
        let x = rng.gen_range(1, m_width - w - 1);
        let y = rng.gen_range(1, m_height - h - 1);

        let new_room = RectRoom::new(x, y, w, h);

        // Carve this room (changing walls to floor space)
        for room_y in new_room.y..(new_room.y + new_room.height) {
            for room_x in new_room.x..(new_room.x + new_room.width) {
                map[room_y as usize][room_x as usize] = 0;
            }
        }

        // 3. Connect this new room to the previous room
        if !placed_rooms.is_empty() {
            let (new_cx, new_cy) = new_room.center();
            let (prev_cx, prev_cy) = placed_rooms[placed_rooms.len() - 1].center();

            // Interleave tunnel directionality choices using our local generator
            if rng.gen_range(0, 1) == 1 {
                carve_horizontal_tunnel(&mut map, prev_cx, new_cx, prev_cy);
                carve_vertical_tunnel(&mut map, prev_cy, new_cy, new_cx);
            } else {
                carve_vertical_tunnel(&mut map, prev_cy, new_cy, prev_cx);
                carve_horizontal_tunnel(&mut map, prev_cx, new_cx, new_cy);
            }
        }

        placed_rooms.push(new_room);
    }

    map
}

fn carve_horizontal_tunnel(map: &mut Map, x1: i32, x2: i32, y: i32) {
    let start = x1.min(x2);
    let end = x1.max(x2);
    for x in start..=end {
        map[y as usize][x as usize] = 0;
    }
}

fn carve_vertical_tunnel(map: &mut Map, y1: i32, y2: i32, x: i32) {
    let start = y1.min(y2);
    let end = y1.max(y2);
    for y in start..=end {
        map[y as usize][x as usize] = 0;
    }
}

pub fn print_map(map: &Map) {
    for row in map.iter() {
        for value in row.iter() {
            print!("{} ", value);
        }
        println!();
    }
}

pub fn draw_map(d: &mut RaylibDrawHandle, map: &Map, cell_size: i32) {
    for (y, row) in map.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            let screen_x = x as i32 * cell_size;
            let screen_y = y as i32 * cell_size;

            if *value == 1 {
                d.draw_rectangle(screen_x, screen_y, cell_size, cell_size, Color::GREEN);
            }
            d.draw_rectangle_lines(screen_x, screen_y, cell_size, cell_size, Color::GRAY);
        }
    }
}

