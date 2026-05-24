use raylib::math::Vector2;

pub struct Player {
    pub pos: Vector2,
    pub dir: Vector2,
    pub velocity: Vector2,
}

impl Player {
    pub fn new(pos: Vector2, dir: Vector2, velocity: Vector2) -> Self {
        Self {
            pos: pos,
            dir: dir,
            velocity: velocity,
        }
    }

    pub fn update_position(&mut self, dt: f32) {
        self.pos.x += self.velocity.x * dt;
        self.pos.y += self.velocity.y * dt;
    }
}
