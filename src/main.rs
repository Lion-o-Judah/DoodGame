mod world;
mod dood;

use world::map;

fn main() {
    let my_dood = dood::Dood::new();
    println!("{}", my_dood.sex);
    match map::Map::random_map() {
        Ok(world) => {println!("{}", world);},
        Err(error) => {println!("there was an error generating the world: {}", error)},
    };
    
}
