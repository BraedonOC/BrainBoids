use nannou::prelude::*;
use crate::boids::*;
use std::{thread, time};
use rand::Rng;

mod boids;

const BORDER_BUFFER: f32 = 5.0;

const BOIDS: usize = 300;

const DRAW_NEIGHBORS: bool = true;
const DRAW_VELS: bool = false;

const SPECIES_COLORS: [(u8, u8, u8); NUMBER_OF_SPECIES] = [
                                                             (250, 230, 100),
                                                             (100, 255, 200),
                                                             (100, 200, 255),
                                                          ];





// impl From<Color> for Rgb {
//     fn from(c: Color) -> Self {
//         named::from_str(&c.to_string()).unwrap()
//     }
// }

fn main() {
    nannou::app(model)
            .update(update)
            .run();
}


struct Model {
    _window : window::Id,
    population : Vec<Boid>,
}



fn model(app: &App) -> Model {

    let mut population = Vec::with_capacity(BOIDS);
    let mut rng = rand::thread_rng();

    for _ in 0..BOIDS {
        population.push(Boid{ pos     : Position::default(), 
                              vel     : Velocity::default(),
                              species : rng.gen_range(0..NUMBER_OF_SPECIES),
                            });
    }

    let _window = app.new_window()
                     .view(view)
                     .build()
                     .unwrap();

    Model {_window, population}
}

fn update(app : &App, model: &mut Model, _update: Update) {
    
    thread::sleep(time::Duration::from_millis(10));
    for b in 0..model.population.len() {
        let boid = &model.population[b];
        // TODO: this math is wrong with the mags
        let (mut new_pos_x, mut new_pos_y) = (boid.pos.x + boid.vel.x,  boid.pos.y + boid.vel.y);

        let boundry = app.window_rect();

        if new_pos_x.abs() > boundry.right() {
            new_pos_x = if new_pos_x > 0.0 {boundry.left() + BORDER_BUFFER} else {boundry.right() - BORDER_BUFFER};
        }

        if new_pos_y.abs() > boundry.top() {
            new_pos_y = if new_pos_y > 0.0 {boundry.bottom() + BORDER_BUFFER} else {boundry.top() - BORDER_BUFFER};
        }

        let updated_vel = adjust_vel(b, &model.population);
        let updated_boid = Boid { pos : Position{x : new_pos_x, y : new_pos_y},
                                  vel : updated_vel,
                                  species : boid.species };

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
        let boid = &model.population[b];
        let boid_pos = pt2(boid.pos.x, boid.pos.y);

        let mut species_color = Rgb::default();
        (   species_color.red,
            species_color.green,
            species_color.blue) = SPECIES_COLORS[boid.species as usize];

        draw.ellipse().w(10.0).h(10.0).color(species_color).x_y(boid.pos.x, boid.pos.y);
 
        if DRAW_VELS {
            let vel_pos = pt2(boid.pos.x + boid.vel.x, boid.pos.y + boid.vel.y);
            let speed = f32::sqrt( 
                                       f32::powi(boid.vel.x, 2)
                                     + f32::powi(boid.vel.y, 2)
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
                let neighbor = &model.population[neighbor_idx];
                if neighbor.species == boid.species {
                    let neighbor_pos = pt2(neighbor.pos.x, neighbor.pos.y);
                    draw.line()
                        .start(boid_pos)
                        .end(neighbor_pos)
                        .weight(1.0)
                        .color(species_color);
                }
            }
        }
    }



    let mut background_color = Rgb::default();
    background_color.red   =   ((220) as f32  + 15.0  * (app.time / 4.0 + 0.2).sin()) as u8;
    background_color.green =   ((120 ) as f32 + 30.0  * (app.time / 4.0 + 4.4).sin()) as u8;
    background_color.blue  =   ((150) as f32  +  50.0 * (app.time / 4.0 + 1.1).sin()) as u8;
    draw.background().color(background_color);

    draw.to_frame(app, &frame).unwrap();
}

