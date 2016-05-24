mod room;
mod entity;
mod world;

fn main() {
    let mut world = world::World::new();
    world.addRoom("beep", room::Room::new());
    world.addRoom("beep1", room::Room::new());
    world.addRoom("beep2", room::Room::new());



    println!("Hello, world!");
}
