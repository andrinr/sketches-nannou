mod particle;
mod attractor;

extern crate nannou;

use nannou::prelude::*;
use nannou::glam::Vec2;


const PARTICLE_COUNT : usize = 1<<13;
const N_ATTRACTORS : usize = 10;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(1200, 1200)
        .run();
}


struct Model {
    particles : Vec<particle::Particle>,
    attractors : Vec<attractor::Attractor>,
    i : i32
}

fn model(app: &App) -> Model {
    let window = app.main_window();
    let win = window.rect();

    let h : f32 = win.h() as f32;

    let mut attractors : Vec<attractor::Attractor> = Vec::with_capacity(N_ATTRACTORS);

    for _i in 0..N_ATTRACTORS {

        let a = win.h() * 0.5;

        let position : Vec2 = Vec2::new(
            random_f32() * a - a / 2.0,
            random_f32() * a - a / 2.0
        );

        let strength : f32 = random_f32() * 10.;

        let attractor : attractor::Attractor = attractor::Attractor {
            position : position,
            strength : strength
        };

        attractors.push(attractor);
    }

    let mut particles : Vec<particle::Particle> = Vec::with_capacity(PARTICLE_COUNT);

    for i in 0..PARTICLE_COUNT {
        let r = h / 4.5;

        let position : Vec2 = Vec2::new(
            0.0,
            0.0
        );
        
        let periodic : Vec2 = Vec2::new(
            (i as f32 / PARTICLE_COUNT as f32 * 2.0 * PI * 2.0).cos(),
            (i as f32 / PARTICLE_COUNT as f32 * 2.0 * PI * 2.0).sin()
        );

        let velocity : Vec2 = periodic * r * 0.01 + position * 0.1;

        let particle : particle::Particle = particle::Particle {
            acceleration : Vec2::new(0.0, 0.0),
            radius : 2.0,
            position : position,
            velocity : velocity,
            age : 0.0,
            id : i as i32
        };

        particles.push(particle);
    }

    println!("lengths : {}", particles.len());

    Model {
        particles : particles.to_vec(), 
        attractors : attractors.to_vec(),
        i : 0
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {

    for _i in 0..8 {
        for particle in model.particles.iter_mut() {
            for attractor in model.attractors.iter() {
                particle.impulse(attractor.get_force(particle.position));
            }
            particle.kick_drift_kick(0.01);

            particle.velocity *= 0.97;
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

    // for attractor in model.attractors.iter() {
    //     attractor.draw(&draw);
    // }

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