use crate::space::{Position, Vector};
use rand::Rng;

#[derive(Debug)]
pub struct Entity {
    pub position: Position<f64>,
    pub direction: Vector<f64>,
    pub speed: f64,
}

const SEPARATION_DISTANCE: f64 = 1.0;
const FLOCKING_DISTANCE: f64 = 2.0;

// Should be between 0 and 1
// The higher this is, the more the flock position dominates the individual choices
const FLOCK_SCALE: f64 = 1.0;

impl Entity {
    pub fn new(position: Position<f64>, direction: Vector<f64>, speed: f64) -> Self {
        Entity {
            position,
            direction,
            speed,
        }
    }

    pub fn random<T: Rng>(rng: &mut T) -> Self {
        Self::new(
            Position::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            ),
            Vector::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            ),
            rng.gen(),
        )
    }

    pub fn distance(&mut self, other: &Self) -> f64 {
        self.position.distance_squared(&other.position).sqrt()
    }

    pub fn flock(&mut self, group: &[Entity]) {
        let mut avg_from = vec![self.direction.clone()];

        let too_close = group
            .iter()
            .filter_map(|e| {
                if self.distance(e) < SEPARATION_DISTANCE {
                    Some(Vector::new(0.0, 0.0, 0.0) - e.direction.clone())
                } else {
                    None
                }
            })
            .collect();
        if let Some(away_from) = Vector::average(too_close) {
            avg_from.insert(0, away_from.scale(FLOCK_SCALE));
        }

        let flock = group
            .iter()
            .filter_map(|e| {
                if self.distance(e) >= SEPARATION_DISTANCE && self.distance(e) < FLOCKING_DISTANCE {
                    Some(e.direction.clone())
                } else {
                    None
                }
            })
            .collect();
        if let Some(towards) = Vector::average(flock) {
            avg_from.insert(0, towards.scale(FLOCK_SCALE));
        }

        self.direction = Vector::average(avg_from).unwrap();
    }

    pub fn group_flock(group: &mut [Entity]) {
        for i in 1..group.len() {
            let (a, b) = group.split_at_mut(i);
            let (g, c) = group.split_at_mut(1);
            g.map(|e| e.flock(a.concat(c)));
        }
    }
}
