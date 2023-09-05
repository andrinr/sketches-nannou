mod particle;

extern crate nannou;

use nannou::prelude::*;
use nannou::glam::Vec2;


const PARTICLE_COUNT : usize = 1<<13;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(1200, 1200)
        .run();
}

struct Model {
    particles : Vec<particle::Particle>,
    i : i32
}

fn model(app: &App) -> Model {
    let window = app.main_window();
    let win = window.rect();

    let h : f32 = win.h() as f32;

    let mut particles : Vec<particle::Particle> = Vec::with_capacity(PARTICLE_COUNT);

    let mut particles_indices : Vec<usize> = Vec::with_capacity(PARTICLE_COUNT);
    let mut particles_tracer : Vec<usize> = Vec::with_capacity(PARTICLE_COUNT);

    for i in 0..PARTICLE_COUNT {
        let r = h / 4.5;
        let f: f32 = 0.5;

        let position : Vec2 = Vec2::new(
            (i as f32 / PARTICLE_COUNT as f32 * PI * 2.0).cos() * r,
            (i as f32 / PARTICLE_COUNT as f32 * PI * 2.0).sin() * r,
        );

        let tangent : Vec2 = Vec2::new(
            position.y,
            -position.x
        );

        let periodic : Vec2 = Vec2::new(
            (i as f32 / PARTICLE_COUNT as f32 * 2.0 * PI * 2.0).cos(),
            (i as f32 / PARTICLE_COUNT as f32 * 2.0 * PI * 2.0).sin()
        );

        let velocity : Vec2 = (position * periodic.x + tangent * periodic.y) * 0.05 + position * 0.4 + tangent * 0.1;

        let particle : particle::Particle = particle::Particle {
            acceleration : Vec2::new(0.0, 0.0),
            radius : 2.0,
            position : position,
            velocity : velocity,
            age : 0.0
        };

        particles.push(particle);

        particles_indices.push(i);
        particles_tracer.push(0);
    }

    println!("lengths : {}", particles.len());

    Model {
        particles : particles.to_vec(), 
        i : 0
    }
}

fn update(app: &App, model: &mut Model, update: Update) {

    //let dt : f32 = (update.since_last.subsec_millis() as f32) * 0.001;

    //println!("fps: {}", 1.0 / dt);

    let window = app.main_window();
    let win = window.rect();

    for i in 0..5 {
        for particle in model.particles.iter_mut() {
            particle.kick_drift_kick(0.005);
            //particle.enforce_boundary_conditions(w, h);
            // Damping as no energy conversion in system
            //particle.velocity *= 0.999;
        }
    }

    model.i += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // set background to blue
    draw.rect().wh(frame.rect().wh()).color(Rgba::new(0.0, 0.0, 0.0, 0.01));
    for particle in model.particles.iter() {
        particle.draw(&draw);
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();

    let file_path = captured_frame_path(app, &frame);
    app.main_window().capture_frame(file_path);
}

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    // Create a path that we want to save this frame to.
    app.project_path()
        .expect("failed to locate `project_path`")
        // Capture all frames to a directory called `/<path_to_nannou>/nannou/simple_capture`.
        .join("out")
        // Name each file after the number of the frame.
        .join(format!("{:03}", frame.nth()))
        // The extension will be PNG. We also support tiff, bmp, gif, jpeg, webp and some others.
        .with_extension("png")
}