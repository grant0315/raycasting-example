// src/player.rs
use crate::map::Map;
use raylib::math::Vector2;

pub struct Player {
    pub pos: Vector2,
    pub dir: Vector2,
    pub angle: f32,
    pub fov: f32,
}

impl Player {
    pub fn new(pos: Vector2, initial_dir: Vector2, fov: f32) -> Self {
        let angle = initial_dir.y.atan2(initial_dir.x);
        Self {
            pos,
            dir: initial_dir.normalized(),
            angle,
            fov,
        }
    }

    pub fn update(
        &mut self,
        move_speed: f32,
        turn_speed: f32,
        level: &Map,
        cell_size: i32,
        dt: f32,
    ) {
        // 1. Process rotation angle updates
        self.angle += turn_speed * dt;
        const TWO_PI: f32 = 2.0 * std::f32::consts::PI;
        if self.angle < 0.0 {
            self.angle += TWO_PI;
        }
        if self.angle > TWO_PI {
            self.angle -= TWO_PI;
        }

        self.dir.x = self.angle.cos();
        self.dir.y = self.angle.sin();

        // 2. Calculate potential new position positions
        let delta_x = self.dir.x * move_speed * dt;
        let delta_y = self.dir.y * move_speed * dt;

        let next_pos_x = self.pos.x + delta_x;
        let next_pos_y = self.pos.y + delta_y;

        // Bounding collision buffer space padding
        // This keeps your view camera from getting close to the walls
        let buffer = 10.0;
        let check_sign_x = if delta_x > 0.0 { buffer } else { -buffer };
        let check_sign_y = if delta_y > 0.0 { buffer } else { -buffer };

        // --- SEPARATE AXIS COLLISION TESTING ---

        // Test X-Axis Movement independently
        let grid_x = ((next_pos_x + check_sign_x) / cell_size as f32).floor() as i32;
        let current_grid_y = (self.pos.y / cell_size as f32).floor() as i32;

        if grid_x >= 0
            && grid_x < level[0].len() as i32
            && current_grid_y >= 0
            && current_grid_y < level.len() as i32
        {
            if level[current_grid_y as usize][grid_x as usize] == 0 {
                self.pos.x = next_pos_x; // Safe to apply X movement step
            }
        }

        // Test Y-Axis Movement independently (Allows sliding functionality)
        let current_grid_x = (self.pos.x / cell_size as f32).floor() as i32;
        let grid_y = ((next_pos_y + check_sign_y) / cell_size as f32).floor() as i32;

        if current_grid_x >= 0
            && current_grid_x < level[0].len() as i32
            && grid_y >= 0
            && grid_y < level.len() as i32
        {
            if level[grid_y as usize][current_grid_x as usize] == 0 {
                self.pos.y = next_pos_y; // Safe to apply Y movement step
            }
        }
    }
}

