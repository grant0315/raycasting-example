use raylib::math::Vector2;
use raylib::prelude::*;

pub struct Player {
    pub pos: Vector2,
    pub dir: Vector2,
    pub velocity: Vector2,
    pub fov: f32,
}

impl Player {
    pub fn new(pos: Vector2, dir: Vector2, velocity: Vector2, fov: f32) -> Self {
        Self {
            pos: pos,
            dir: dir,
            velocity: velocity,
            fov: fov,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.pos.x += self.velocity.x * dt;
        self.pos.y += self.velocity.y * dt;

        if self.velocity.length() > 0.0 {
            self.dir = self.velocity.normalized();
        }
    }
}
