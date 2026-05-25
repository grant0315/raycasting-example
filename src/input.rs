// src/input.rs
use raylib::prelude::*;

// Configuration parameters
const MOVE_SPEED_MAX: f32 = 250.0; // Pixels per second
const ROTATION_SPEED: f32 = 4.5; // Radians per second

pub struct PlayerInputIntent {
    pub move_speed: f32,
    pub turn_speed: f32,
}

pub fn poll_player_movement_input(rl: &RaylibHandle) -> PlayerInputIntent {
    let mut move_speed = 0.0;
    let mut turn_speed = 0.0;

    // W / S: Forward & Backward
    if rl.is_key_down(KeyboardKey::KEY_W) {
        move_speed += MOVE_SPEED_MAX;
    }
    if rl.is_key_down(KeyboardKey::KEY_S) {
        move_speed -= MOVE_SPEED_MAX;
    }

    // A / D: Rotate Left & Rotate Right
    if rl.is_key_down(KeyboardKey::KEY_A) {
        turn_speed -= ROTATION_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_D) {
        turn_speed += ROTATION_SPEED;
    }

    PlayerInputIntent {
        move_speed,
        turn_speed,
    }
}
