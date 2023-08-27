use rand::Rng;
use std::default::Default;


const X_SPAWN_AREA: f32 = 500.0;
const Y_SPAWN_AREA: f32 = 500.0;
const NEIGHBORS: u16 = 5;
const TARGET_DISTANCE: f32 = 40.0;

const DISGUST: f32 = 0.005;
const WALL_FEAR: f32 = 1.01;

const MAX_SPEED: f32 = 5.0;
const MIN_SPEED: f32 = 3.0;

const SLOW_POINT: f32 = 5.0;

pub const NUMBER_OF_SPECIES: usize = 3;

const RELATIONSHIPS: [[f32; NUMBER_OF_SPECIES]; NUMBER_OF_SPECIES] = 
                     [
                        [  4.0,  100.0,  100.0],
                        [ 10.0, 1.0, 10.0],
                        [ 0.1,  1.0, 0.8],
                     ];

pub struct Position {
    pub x : f32,
    pub y : f32,
}


impl Default for Position {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        Position {
                    x : (rng.gen::<f32>() * 2.0 * X_SPAWN_AREA) - (X_SPAWN_AREA),
                    y : (rng.gen::<f32>() * 2.0 * Y_SPAWN_AREA) - (Y_SPAWN_AREA),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Velocity {
    pub x   : f32,
    pub y   : f32,
}

impl Default for Velocity {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        Velocity {
                    x   : rng.gen::<f32>(),
                    y   : rng.gen::<f32>(),
        }
    }
}

pub struct Boid {
    pub pos     : Position,
    pub vel     : Velocity,
    pub species : usize
}

/*
impl Clone for MyStruct {
    fn clone(&self) -> MyStruct {
        *self
    }
}
*/

pub fn nearest_neighbors(boid_index : usize, boids : &Vec<Boid>) -> Vec<(usize, f32)>  {
    let cur_boid = &boids[boid_index];
    let mut nearest : Vec<(usize, f32)> = Vec::new();
    for b in 1..boids.len() {
        if b == boid_index {
            continue;
        }
        let other_boid = &boids[b];
        let dist = f32::sqrt( 
                               f32::powi(cur_boid.pos.x - other_boid.pos.x, 2)
                             + f32::powi(cur_boid.pos.y - other_boid.pos.y, 2)
                            );
        if (nearest.len() as u16) < NEIGHBORS {
            nearest.push((b, dist));
        }
        else {
            for i in 0..nearest.len() {
                let (_, dist_in_question) = nearest[i];
                if dist < dist_in_question{
                    nearest[i] = (b, dist);
                    break;
                }
            }
        }
    }
    nearest
}

pub fn adjust_vel(boid_index : usize, boids : &Vec<Boid>) -> Velocity {
    let mut neighbor_x_sum : f32 = 0.0;
    let mut neighbor_y_sum : f32 = 0.0;

    let mut avoidance_x_sum : f32 = 0.0;
    let mut avoidance_y_sum : f32 = 0.0;

    let cur_boid = &boids[boid_index];

    let neighbors = nearest_neighbors(boid_index, boids);

    let mut same_species_neighbors : u8 = 0;


    for descriptor in neighbors {

        let (idx, dist) = descriptor;
        let neighbor = &boids[idx];
        
        // positive avoidance factor implies we need to move away
        let avoidance_factor =   f32::powi((dist - RELATIONSHIPS[cur_boid.species][neighbor.species] * TARGET_DISTANCE) / SLOW_POINT, 3);
        avoidance_x_sum += (neighbor.pos.x - cur_boid.pos.x) * avoidance_factor * DISGUST;
        avoidance_y_sum += (neighbor.pos.y - cur_boid.pos.y) * avoidance_factor * DISGUST;

        avoidance_x_sum += if cur_boid.pos.x > 0.0 {-1.0} else {1.0} * f32::powf(WALL_FEAR, cur_boid.pos.x.abs() - X_SPAWN_AREA);
        avoidance_y_sum += if cur_boid.pos.y > 0.0 {-1.0} else {1.0} * f32::powf(WALL_FEAR, cur_boid.pos.y.abs() - Y_SPAWN_AREA);

        if neighbor.species == cur_boid.species {
            same_species_neighbors += 1;
            neighbor_x_sum += neighbor.vel.x * dist;
            neighbor_y_sum += neighbor.vel.y * dist;
        }
    }

    if same_species_neighbors > 0 {
        neighbor_x_sum = neighbor_x_sum / (same_species_neighbors as f32);
        neighbor_y_sum = neighbor_y_sum / (same_species_neighbors as f32);
    }
    let mut new_vel_x : f32 = neighbor_x_sum + avoidance_x_sum;
    let mut new_vel_y : f32 = neighbor_y_sum + avoidance_y_sum;

    let new_speed = f32::sqrt( 
                               f32::powi(new_vel_x, 2)
                             + f32::powi(new_vel_y, 2)
                           );

    if new_speed > MAX_SPEED {
        new_vel_x = new_vel_x * MAX_SPEED / new_speed;
        new_vel_y = new_vel_y * MAX_SPEED / new_speed;
    }

    if new_speed < MIN_SPEED {
        new_vel_x = new_vel_x * MIN_SPEED / new_speed;
        new_vel_y = new_vel_y * MIN_SPEED / new_speed;
    }

    let adjusted_speed =  f32::sqrt( 
                               f32::powi(new_vel_x, 2)
                             + f32::powi(new_vel_y, 2)
                           );

    println!("in adjust vec boid {} has vel ({}, {}) speed {}", boid_index, new_vel_x, new_vel_y, adjusted_speed);

    Velocity{x: new_vel_x, y: new_vel_y}
}

