
use nannou::glam::Vec2;
use nannou::prelude::*;

const RADIUS : f32 = 5.0;

#[derive(Copy, Clone)]
pub struct Attractor {
    pub position : Vec2,
    pub strength : f32
}

impl Attractor {
    pub fn get_force(self, other : Vec2) -> Vec2 {
        let vector = self.position - other;
        let d : f32 = vector.length() + 0.5;
        return vector.clone().normalize() / d * self.strength;
    }

    pub fn draw(self, draw : &Draw) {
        draw.ellipse()
        .color(RED)
        .x_y(self.position.x, self.position.y)
        .radius(RADIUS);
    }
}

