use crate::{input::InputHandler, render::vehicle};

pub fn traffic_manager(input: &mut InputHandler, vehicles: &mut Vec<vehicle::Vehicle>) {
    input.spawn_cars(vehicles);

    // update vehicles
    for vehicle in vehicles {
        // todo: check and handle for possible collisions for each vehicle
        vehicle.update();
    }
}
