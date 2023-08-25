mod entity;
mod space;

use crate::entity::Entity;
use crate::space::Vector;

fn main() {
    let mut rng = rand::thread_rng();
    let e1 = Entity::random(&mut rng);
    println!("e1 direction: {:?}", e1.direction);
    let e2 = Entity::random(&mut rng);
    println!("e2 direction: {:?}", e2.direction);
    let avg = Vector::average(vec![e1.direction.clone(), e2.direction.clone()]);
    println!("Average: {:?}", avg);
    let mut e3 = Entity::random(&mut rng);
    println!("Before flock: {:?}", e3);
    let flock = vec![e1, e2, e3];

    // e3.flock(&flock);
    println!("After flock: {:?}", e3);

    let a: u8 = 0;
    let b = (a as i8 - 1) as u8;
    println!("{}", b);
}
