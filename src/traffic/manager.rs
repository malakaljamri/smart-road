use crate::{input::InputHandler, render::{vehicle, Statistics}, simulation::Simulation};
use crate::traffic::Collision;

pub fn traffic_manager(input: &mut InputHandler, vehicles: &mut Vec<vehicle::Vehicle>, statistics: &mut Statistics, completed_vehicles: &mut Vec<vehicle::Vehicle>, simulation: &mut Simulation) {
    input.spawn_cars(vehicles, simulation);

    // Track vehicles to remove
    let mut vehicles_to_remove: Vec<usize> = Vec::new();

    // update vehicles
    for i in 0..vehicles.len() {
        // Create a temporary reference to avoid borrowing issues
        let vehicles_clone = vehicles.clone();
        vehicles[i].update(&vehicles_clone);
        
        // Check for close calls (vehicles within safe distance)
        for other in &vehicles_clone {
            if other.id != vehicles[i].id {
                if Collision::is_vehicle_in_intersection(&vehicles[i]) || Collision::is_vehicle_in_intersection(other) {
                    if Collision::is_vehicle_in_path(&vehicles[i], other, vehicles[i].collision.safe_distance) {
                        vehicles[i].had_close_call = true;
                        println!("close one there buddy")
                    }
                }
            }
        }
        
        // Remove vehicles that have exited screen (completed intersection)
        if vehicles[i].x < -50.0 || vehicles[i].x > 850.0 || vehicles[i].y < -50.0 || vehicles[i].y > 850.0 {
            vehicles_to_remove.push(i);
            completed_vehicles.push(vehicles[i].clone());
        }
    }
    
    // Remove completed vehicles (in reverse order to maintain indices)
    for &index in vehicles_to_remove.iter().rev() {
        vehicles.remove(index);
    }
    
    // Update statistics with completed vehicles
    statistics.calculate_intersection_stats(completed_vehicles);
}
