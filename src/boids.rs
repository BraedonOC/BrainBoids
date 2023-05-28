use rand::Rng;
use std::default::Default;


const X_SPAWN_AREA: f32 = 500.0;
const Y_SPAWN_AREA: f32 = 500.0;
const NEIGHBORS: u16 = 5;
const TARGET_DISTANCE: f32 = 30.0;
const DISGUST: f32 = 0.05;
const MAX_SPEED: f32 = 5.0;



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

pub fn nearest_neighbors(boid_index : usize, boids : &Vec<(Position, Velocity)>) -> Vec<(usize, f32)>  {
    let (target_pos, _) = &boids[boid_index];
    let mut nearest : Vec<(usize, f32)> = Vec::new();
    for b in 1..boids.len() {
        if b == boid_index {
            continue;
        }
        let (b_pos, _) = &boids[b];
        let dist = f32::sqrt( 
                               f32::powi(target_pos.x - b_pos.x, 2)
                             + f32::powi(target_pos.y - b_pos.y, 2)
                            );
//      println!("Dist between {}, {} and {}, {} = {}", b_pos.x, b_pos.y, target_pos.x, target_pos.y, dist);
        if (nearest.len() as u16) < NEIGHBORS {
            nearest.push((b, dist));
        } 
        else {
            for i in 0..nearest.len() {
                let (_, dist_in_question) = nearest[i];
                if dist < dist_in_question{
                    nearest[i] = (b, dist);
                }
            }
        }
    }
    nearest
}

pub fn adjust_vel(boid_index : usize, boids : &Vec<(Position, Velocity)>) -> Velocity {
    let mut neighbor_x_sum : f32 = 0.0;
    let mut neighbor_y_sum : f32 = 0.0;

    let mut avoidance_x_sum : f32 = 0.0;
    let mut avoidance_y_sum : f32 = 0.0;

    let (cur_pos, _) = &boids[boid_index];

    let neighbors = nearest_neighbors(boid_index, boids);

    for descriptor in neighbors {
        let (idx, dist) = descriptor;
        let (neigh_pos, neigh_vel) = &boids[idx];
        
        // positive avoidance factor implies we need to move away
        let avoidance_factor =  dist - TARGET_DISTANCE; 
        avoidance_x_sum += (neigh_pos.x - cur_pos.x) * avoidance_factor * DISGUST;
        avoidance_y_sum += (neigh_pos.y - cur_pos.y) * avoidance_factor * DISGUST;

        println!("neighbor dist: {}", dist);

        println!("avoidance_x_sum from neighbor: {}", avoidance_x_sum);
        println!("avoidance_y_sum from neighbor: {}", avoidance_y_sum);

//      if cur_pos.x > X_SPAWN_AREA {
//          avoidance_x_sum += X_SPAWN_AREA - cur_pos.x;
//      }
//      if cur_pos.x < -X_SPAWN_AREA {
//          avoidance_x_sum += X_SPAWN_AREA - cur_pos.x;
//      }
//      if cur_pos.y > Y_SPAWN_AREA {
//          avoidance_y_sum += Y_SPAWN_AREA - cur_pos.y;
//      }
//      if cur_pos.y < -Y_SPAWN_AREA {
//          avoidance_y_sum += Y_SPAWN_AREA - cur_pos.y;
//      }

//      println!("avoidance_x_sum after spawn area: {}", avoidance_x_sum);
//      println!("avoidance_y_sum after spawn area: {}", avoidance_y_sum);

        neighbor_x_sum += neigh_vel.x;
        neighbor_y_sum += neigh_vel.y;
        
    }

    let matching_vel_x = neighbor_x_sum / (NEIGHBORS as f32);
    let matching_vel_y = neighbor_y_sum / (NEIGHBORS as f32);
    println!("adjusting boid vel to <{}, {}>", matching_vel_x + avoidance_x_sum, matching_vel_y + avoidance_y_sum);
    let mut new_vel_x : f32 = matching_vel_x + avoidance_x_sum;
    let mut new_vel_y : f32 = matching_vel_y + avoidance_y_sum;

    let new_speed = f32::sqrt( 
                               f32::powi(new_vel_x, 2)
                             + f32::powi(new_vel_x, 2)
                           );
    if new_speed > MAX_SPEED {
        new_vel_x = new_vel_x * MAX_SPEED / new_speed;
        new_vel_y = new_vel_y * MAX_SPEED / new_speed;
    }

    let adjusted_speed =  f32::sqrt( 
                               f32::powi(new_vel_x, 2)
                             + f32::powi(new_vel_x, 2)
                           );

    println!("normalized vel <{}, {}> has mag {}", new_vel_x, new_vel_y, adjusted_speed);

    Velocity{x: new_vel_x, y: new_vel_y}
}

