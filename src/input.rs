use raylib::prelude::*;

const SPEED: f32 = 2.0;

pub fn poll_player_movement_input(rl: &RaylibHandle, initial_velocity: Vector2) -> Vector2 {
    let mut resultant_velocity: Vector2 = initial_velocity;

    if rl.is_key_pressed(KeyboardKey::KEY_W) {
        resultant_velocity.x -= SPEED;
    } else if rl.is_key_pressed(KeyboardKey::KEY_S) {
        resultant_velocity.x += SPEED;
    } else if rl.is_key_pressed(KeyboardKey::KEY_A) {
        resultant_velocity.y -= SPEED;
    } else if rl.is_key_pressed(KeyboardKey::KEY_D) {
        resultant_velocity.y += SPEED;
    }

    resultant_velocity
}
