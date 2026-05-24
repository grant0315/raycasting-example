use raylib::prelude::*;

const SPEED: f32 = 100.0;

pub fn poll_player_movement_input(rl: &RaylibHandle) -> Vector2 {
    let mut resultant_velocity: Vector2 = Vector2::zero();

    if rl.is_key_down(KeyboardKey::KEY_W) {
        resultant_velocity.y -= SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_S) {
        resultant_velocity.y += SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_A) {
        resultant_velocity.x -= SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_D) {
        resultant_velocity.x += SPEED;
    }

    resultant_velocity
}
