use nannou::glam::Vec2;
use nannou::prelude::*;

#[derive(Copy, Clone)]
pub struct Particle {
    pub id : i32,
    pub position : Vec2,
    pub velocity : Vec2,
    pub acceleration : Vec2,
    pub radius : f32,
    pub age : f32,
}

impl Particle {
    pub fn impulse(&mut self, vector : Vec2) {
        self.acceleration += vector;
    }

    pub fn kick_drift_kick(&mut self, dt : f32) {
        self.age += dt;
        // Leap-Frog Integration
        // Kick
        let v_half = self.velocity + self.acceleration * dt * 0.5;
        // Drift
        self.position += v_half * dt;
        // Kick
        self.velocity = v_half + self.acceleration * dt * 0.5;
    }

    pub fn draw(self, draw : &Draw) {
        draw.ellipse()
        .hsl(self.id as f32 / 1000., 1.0, 0.5)
        .x_y(self.position.x, self.position.y)
        .radius(self.radius);
    }
}