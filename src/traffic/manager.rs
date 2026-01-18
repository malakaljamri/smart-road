use crate::{input::InputHandler, render::vehicle};

pub fn traffic_manager(input: &mut InputHandler, vehicles: &mut Vec<vehicle::Vehicle>) {
    input.spawn_cars(vehicles);

    // update vehicles
    for i in 0..vehicles.len() {
        // Create a temporary reference to avoid borrowing issues
        let vehicles_clone = vehicles.clone();
        vehicles[i].update(&vehicles_clone);
    }
}
