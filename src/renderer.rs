// src/renderer.rs
use crate::map::Map;
use crate::raycasting::{self, Ray};
use raylib::math::{Rectangle, Vector2};
use raylib::prelude::*;

pub fn render_pseudo_3d(
    d: &mut RaylibDrawHandle,
    level: &Map,
    texture: &Texture2D,
    player_pos: Vector2,
    player_dir: Vector2,
    fov_degrees: f32,
    map_width: i32,
    map_height: i32,
    cell_size: i32,
    screen_width: i32,
    screen_height: i32,
) {
    let half_width = screen_width / 2;

    // Draw background Ceiling & Floor
    d.draw_rectangle(
        half_width,
        0,
        half_width,
        screen_height / 2,
        Color::DARKGRAY,
    );
    d.draw_rectangle(
        half_width,
        screen_height / 2,
        half_width,
        screen_height / 2,
        Color::BROWN,
    );

    let fov_rad = fov_degrees.to_radians();
    let plane_length = (fov_rad / 2.0).tan();
    let camera_plane = Vector2::new(-player_dir.y, player_dir.x) * plane_length;

    let column_width = (half_width as f32 / half_width as f32).ceil() as i32;

    for x in 0..half_width {
        let camera_x = 2.0 * (x as f32 / half_width as f32) - 1.0;
        let ray_dir = player_dir + (camera_plane * camera_x);

        let current_ray = Ray {
            pos: player_pos,
            dir: ray_dir.normalized(),
        };

        if let Some(hit) =
            raycasting::cast_ray_grid(&current_ray, level, map_width, map_height, cell_size)
        {
            let ray_dot_product = current_ray.dir.dot(player_dir);

            // Correct the raw Euclidean distance by projecting it onto the player's view vector [ch_02_04_04]
            let perpendicular_distance = hit.distance * ray_dot_product;
            let safe_dist = if perpendicular_distance < 0.1 {
                0.1
            } else {
                perpendicular_distance
            };

            // Compute wall slice heights using the corrected perpendicular distance value
            let project_wall_height = ((cell_size as f32 * 4.0) / safe_dist) * screen_height as f32;
            let raw_wall_height = project_wall_height as i32;

            // --- VERTICAL CLIPPING PATH ---
            let mut draw_start_y = (screen_height / 2) - (raw_wall_height / 2);
            let mut draw_end_y = (screen_height / 2) + (raw_wall_height / 2);

            let mut texture_source_y_offset = 0.0;
            let mut texture_source_height = texture.height() as f32;

            if draw_start_y < 0 {
                let overflow_pixels = -draw_start_y;
                texture_source_y_offset =
                    (overflow_pixels as f32 / raw_wall_height as f32) * texture.height() as f32;
                draw_start_y = 0;
            }

            if draw_end_y > screen_height {
                let overflow_pixels = draw_end_y - screen_height;
                let chopped_texture_height =
                    (overflow_pixels as f32 / raw_wall_height as f32) * texture.height() as f32;
                texture_source_height -= chopped_texture_height;
                draw_end_y = screen_height;
            }

            texture_source_height -= texture_source_y_offset;
            let final_display_height = draw_end_y - draw_start_y;

            // --- TEXTURE SEGMENT EXTRACTION ---
            let exact_hit_x = player_pos.x + current_ray.dir.x * hit.distance;
            let exact_hit_y = player_pos.y + current_ray.dir.y * hit.distance;

            let mut wall_hit_ratio = if hit.side == 0 {
                exact_hit_y / cell_size as f32
            } else {
                exact_hit_x / cell_size as f32
            };
            wall_hit_ratio -= wall_hit_ratio.floor();

            let tex_x = (wall_hit_ratio * texture.width() as f32) as i32;
            let mut final_tex_x = tex_x;
            if (hit.side == 0 && current_ray.dir.x > 0.0)
                || (hit.side == 1 && current_ray.dir.y < 0.0)
            {
                final_tex_x = texture.width() - tex_x - 1;
            }

            let source_rec = Rectangle::new(
                final_tex_x as f32,
                texture_source_y_offset,
                1.0,
                texture_source_height,
            );

            let dest_rec = Rectangle::new(
                (half_width + x) as f32,
                draw_start_y as f32,
                column_width as f32,
                final_display_height as f32,
            );

            // Shading fog attenuation (Use perpendicular distance here for uniform shading across a flat wall)
            let max_render_depth = 400.0;
            let depth_factor = (1.0 - (perpendicular_distance / max_render_depth)).clamp(0.05, 1.0);
            let side_tint = if hit.side == 0 { 0.5 } else { 1.0 };

            let final_tint_value = (255.0 * depth_factor * side_tint) as u8;
            let render_tint = Color::new(final_tint_value, final_tint_value, final_tint_value, 255);

            d.draw_texture_pro(
                texture,
                source_rec,
                dest_rec,
                Vector2::zero(),
                0.0,
                render_tint,
            );

            if x % 25 == 0 {
                let ray_end = player_pos + (current_ray.dir * hit.distance);
                d.draw_line_v(player_pos, ray_end, Color::new(255, 255, 0, 30));
            }
        }
    }

    d.draw_line(half_width, 0, half_width, screen_height, Color::RAYWHITE);
}
