// src/raycasting.rs
use crate::map::Map;
use raylib::math::Vector2; // Fixed path

pub struct Ray {
    pub pos: Vector2,
    pub dir: Vector2,
}

pub struct RayHit {
    pub distance: f32,
    pub map_x: i32,
    pub map_y: i32,
    pub side: i32,
}

pub fn cast_ray_grid(
    ray: &Ray,
    level: &Map,
    map_width: i32,
    map_height: i32,
    cell_size: i32,
) -> Option<RayHit> {
    let dir_x = if ray.dir.x == 0.0 { 1e-30 } else { ray.dir.x };
    let dir_y = if ray.dir.y == 0.0 { 1e-30 } else { ray.dir.y };

    let mut map_x = (ray.pos.x / cell_size as f32).floor() as i32;
    let mut map_y = (ray.pos.y / cell_size as f32).floor() as i32;

    let delta_dist_x = (1.0 / dir_x).abs();
    let delta_dist_y = (1.0 / dir_y).abs();

    let step_x: i32;
    let step_y: i32;
    let mut side_dist_x: f32;
    let mut side_dist_y: f32;

    if dir_x < 0.0 {
        step_x = -1;
        side_dist_x = ((ray.pos.x / cell_size as f32) - map_x as f32) * delta_dist_x;
    } else {
        step_x = 1;
        side_dist_x = ((map_x + 1) as f32 - (ray.pos.x / cell_size as f32)) * delta_dist_x;
    }

    if dir_y < 0.0 {
        step_y = -1;
        side_dist_y = ((ray.pos.y / cell_size as f32) - map_y as f32) * delta_dist_y;
    } else {
        step_y = 1;
        side_dist_y = ((map_y + 1) as f32 - (ray.pos.y / cell_size as f32)) * delta_dist_y;
    }

    let mut hit = false;
    let mut side = 0;

    while !hit {
        if side_dist_x < side_dist_y {
            side_dist_x += delta_dist_x;
            map_x += step_x;
            side = 0;
        } else {
            side_dist_y += delta_dist_y;
            map_y += step_y;
            side = 1;
        }

        if map_x < 0 || map_x >= map_width || map_y < 0 || map_y >= map_height {
            return None;
        }

        if level[map_y as usize][map_x as usize] > 0 {
            hit = true;
        }
    }

    let grid_distance = if side == 0 {
        side_dist_x - delta_dist_x
    } else {
        side_dist_y - delta_dist_y
    };
    let distance = grid_distance * cell_size as f32;

    Some(RayHit {
        distance,
        map_x,
        map_y,
        side,
    })
}

