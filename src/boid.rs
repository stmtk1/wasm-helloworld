use crate::vector::Vector;
use crate::rand::Rand;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct Boid {
    pub position: Vector,
    pub velocity: Vector,
}

const VELOCITY_SIZE: f64 = 10.0;
const WIDTH: f64 = 600.0;
const HEIGHT: f64 = 400.0;

const ALIGNMENT_RADIOUS: f64 = 50.0;
const ALIGNMENT_WEIGHT: f64 = 3.0;

const SEPARATION_RADIOUS: f64 = 100.0;
const SEPARATION_WEIGHT: f64 = 5.0;

const COHENSION_RADIOUS: f64 = 25.0;
const COHENSION_WEIGHT: f64 = 2.0;

impl Boid {
    pub fn new(rnd_gen: &mut Rand) -> Boid {
        let theta = rnd_gen.random();
        let mut velocity = Vector::new(theta.cos(), theta.sin());
        velocity.mul(VELOCITY_SIZE);

        Boid {
            position: Vector::new(rnd_gen.random() * WIDTH, rnd_gen.random() * HEIGHT),
            velocity,
        }
    }

    pub fn next_state(&self, boids: &Vec<Boid>) -> Boid {
        let mut position = self.position.clone();
        position.add(&self.velocity);

        if position.x < 0.0 {
            position.x += WIDTH;
        } else if position.x > WIDTH {
            position.x -= WIDTH;
        }

        if position.y < 0.0 {
            position.y += HEIGHT;
        } else if position.y > HEIGHT {
            position.y -= HEIGHT;
        }

        let mut velocity = self.velocity.clone();
        velocity.add(&self.alignment(&boids));
        velocity.add(&self.separation(&boids));
        velocity.add(&self.cohension(&boids));
        velocity.normalize();
        velocity.mul(VELOCITY_SIZE);
        Boid { velocity, position, }
    }

    pub fn alignment(&self, boids: &Vec<Boid>) -> Vector {
        let mut ret = Vector::new(0.0, 0.0);
        for boid in boids {
            let distance = boid.position.distance(&self.position);
            if distance == 0.0 || distance >= ALIGNMENT_RADIOUS {
                continue;
            }
            ret.add(&boid.velocity);
        }
        ret.normalize();
        ret.mul(ALIGNMENT_WEIGHT);
        ret
    }

    pub fn separation(&self, boids: &Vec<Boid>) -> Vector {
        let mut ret = Vector::new(0.0, 0.0);
        let position = &self.position;
        for boid in boids {
            let distance = boid.position.distance(&self.position);
            if distance == 0.0 || distance >= SEPARATION_RADIOUS {
                continue;
            }
            let mut sub_vec = boid.position.clone();
            sub_vec.div(distance * distance);
            ret.sub(&sub_vec);
        }
        ret.normalize();
        ret.mul(SEPARATION_WEIGHT);
        ret
    }

    pub fn cohension(&self, boids: &Vec<Boid>) -> Vector {
        let mut ret = Vector::new(0.0, 0.0);
        for boid in boids {
            let distance = boid.position.distance(&self.position);
            if distance == 0.0 || distance >= COHENSION_RADIOUS {
                continue;
            }
            let mut position = boid.position.clone();
            position.sub(&self.position);
            ret.sub(&position);
        }
        ret.normalize();
        ret.mul(COHENSION_WEIGHT);
        ret
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    #[test]
    fn boid_test() {
        let mut rnd = crate::rand::Rand::new(1.0);
        for _ in 0..100 {
            let boid = super::Boid::new(&mut rnd);
            let velocity = boid.velocity;
            let position = boid.position;
            let size = (velocity.x * velocity.x + velocity.y * velocity.y).sqrt();
            writeln!(&mut std::io::stderr(), "{} {}", position.x, position.y).unwrap();
            assert!(((size - 10.0).abs() / size) < 0.00001);
            assert!(0.0 <= position.x && position.x <= super::WIDTH);
            assert!(0.0 <= position.y && position.y <= super::HEIGHT);
        }
        assert!(true);
    }
}
