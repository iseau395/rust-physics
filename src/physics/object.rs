use macroquad::{math::Vec2, prelude::{WHITE, Color}, shapes::draw_circle};

#[derive(Debug, Clone, Copy)]
pub struct Object {
    pub position: Vec2,
    last_position: Vec2,
    acceleration: Vec2,

    pub pinned: bool,

    pub color: Color,

    pub radius: f32,
}

impl Object {
    pub fn new(x: f32, y: f32, radius: f32, color: Color, pinned: bool) -> Object {
        Object {
            position: Vec2::new(x, y),
            last_position: Vec2::new(x, y),
            acceleration: Vec2::new(0., 0.),

            pinned,

            color: color,

            radius,
        }
    }

    pub fn update_position(&mut self, dt: f32) {
        if self.pinned {
            return;
        }

        let velocity = self.position - self.last_position;
        self.last_position = self.position;

        self.position += velocity + self.acceleration * dt * dt;

        self.acceleration = Vec2::new(0., 0.);
    }

    pub fn accelerate(&mut self, accel: Vec2) {
        if self.pinned {
            return;
        }

        self.acceleration += accel;
    }

    pub fn render(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, self.color)
    }
}
