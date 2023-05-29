use nannou::prelude::*;
use crate::boids::Position;
use crate::boids::Velocity;
use crate::boids::adjust_vel;
use crate::boids::nearest_neighbors;
use std::{thread, time};

mod boids;

const BORDER_BUFFER: f32 = 5.0;

const BOIDS: usize = 100;

const DRAW_NEIGHBORS: bool = true;
const DRAW_VELS: bool = true;

fn main() {
    nannou::app(model)
            .update(update)
            .run();
}


struct Model {
    _window : window::Id,
    population : Vec<(Position, Velocity)>,
}



fn model(app: &App) -> Model {

    let mut population = Vec::with_capacity(BOIDS);
    for _ in 0..BOIDS {
        population.push((Position::default(), Velocity::default()));
    }

    let _window = app.new_window()
                     .view(view)
                     .build()
                     .unwrap();

    Model {_window, population}
}

fn update(app : &App, model: &mut Model, _update: Update) {
    
    thread::sleep(time::Duration::from_millis(50));
    for b in 0..model.population.len() {
        let (pos, vel) = &model.population[b];
        // TODO: this math is wrong with the mags
        let (mut new_pos_x, mut new_pos_y) = (pos.x + vel.x,  pos.y + vel.y);

        let boundry = app.window_rect();

        if new_pos_x.abs() > boundry.right() {
            new_pos_x = if new_pos_x > 0.0 {boundry.left() + BORDER_BUFFER} else {boundry.right() - BORDER_BUFFER};
        }

        if new_pos_y.abs() > boundry.top() {
            new_pos_y = if new_pos_y > 0.0 {boundry.bottom() + BORDER_BUFFER} else {boundry.top() - BORDER_BUFFER};
        }

        let updated_vel = adjust_vel(b, &model.population);
        let updated_boid = (Position{x : new_pos_x, y : new_pos_y},
                            updated_vel);

        let speed = f32::sqrt( 
                                   f32::powi(updated_vel.x, 2)
                                 + f32::powi(updated_vel.y, 2)
                             );

        println!("after return boid {} has vel ({}, {}) speed {}", b, updated_vel.x, updated_vel.y, speed);

        model.population[b] = updated_boid;
    }
}


fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    for b in 0..model.population.len() {
        let (pos, vel) = &model.population[b];
        draw.ellipse().w(10.0).h(10.0).color(BLUE).x_y(pos.x, pos.y);
        let boid_pos = pt2(pos.x, pos.y);

        if DRAW_VELS {
            let vel_pos = pt2(pos.x + vel.x, pos.y + vel.y);
            let speed = f32::sqrt( 
                                       f32::powi(vel.x, 2)
                                     + f32::powi(vel.y, 2)
                                 );
            println!("boid {} moving at {}", b, speed);

            draw.line()
                .start(boid_pos)
                .end(vel_pos)
                .weight(3.0)
                .color(STEELBLUE);

        }
        if DRAW_NEIGHBORS {
            for (neighbor_idx, _) in nearest_neighbors(b, &model.population) {
                let (neighbor_pos, _) = &model.population[neighbor_idx];
                let neighbor_pos = pt2(neighbor_pos.x, neighbor_pos.y);
                draw.line()
                    .start(boid_pos)
                    .end(neighbor_pos)
                    .weight(4.0)
                    .color(STEELBLUE);
            }
        }
    }



    draw.background().color(PLUM);

    draw.to_frame(app, &frame).unwrap();
}

