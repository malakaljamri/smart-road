use crate::render::Vehicle;
use crate::types::Direction;

#[derive(Debug, Clone)]
pub struct Collision {
    pub vehicle_id: i32,
    pub x: f32,
    pub y: f32,
    pub safe_distance: f32,
}

impl Collision {
    pub fn new(vehicle_id: i32, x: f32, y: f32) -> Self {
        Self {
            vehicle_id,
            x,
            y,
            safe_distance: 40.0,
        }
    }

    pub fn check_vehicle_ahead<'a>(
        vehicle: &'a Vehicle,
        vehicles: &'a [Vehicle],
    ) -> Option<&'a Vehicle> {
        let check_distance = vehicle.collision.safe_distance + 20.0;

        vehicles.iter().find(|other| {
            other.id != vehicle.id
                && other.lane.from == vehicle.lane.from
                && other.lane.to == vehicle.lane.to
                && Collision::is_vehicle_in_path(vehicle, other, check_distance)
        })
    }

    pub fn is_vehicle_in_path(vehicle: &Vehicle, other: &Vehicle, check_distance: f32) -> bool {
        let distance = ((vehicle.x - other.x).powi(2) + (vehicle.y - other.y).powi(2)).sqrt();

        if distance > check_distance {
            return false;
        }

        match vehicle.direction {
            Direction::North => other.y < vehicle.y && (other.x - vehicle.x).abs() < 20.0,
            Direction::South => other.y > vehicle.y && (other.x - vehicle.x).abs() < 20.0,
            Direction::East => other.x > vehicle.x && (other.y - vehicle.y).abs() < 20.0,
            Direction::West => other.x < vehicle.x && (other.y - vehicle.y).abs() < 20.0,
        }
    }

    pub fn is_vehicle_in_intersection(vehicle: &Vehicle) -> bool {
        // Keep intersection bounds consistent with vehicle logic (295-505 on both axes)
        vehicle.x >= 295.0 && vehicle.x <= 505.0 && vehicle.y >= 295.0 && vehicle.y <= 505.0
    }

    pub fn has_vehicle_in_intersection(vehicles: &[Vehicle], exclude_id: usize) -> bool {
        vehicles
            .iter()
            .any(|v| v.id != exclude_id && Self::is_vehicle_in_intersection(v))
    }

    pub fn should_wait_for_intersection(vehicle: &Vehicle, vehicles: &[Vehicle]) -> bool {
        // Right-turning vehicles can proceed without waiting
        let is_right_turn = match vehicle.lane.from {
            Direction::North => vehicle.lane.to == Direction::West,
            Direction::South => vehicle.lane.to == Direction::East,
            Direction::East => vehicle.lane.to == Direction::North,
            Direction::West => vehicle.lane.to == Direction::South,
        };

        if is_right_turn {
            return false;
        }

        // Straight and left-turning vehicles must wait if intersection is occupied
        Self::has_vehicle_in_intersection(vehicles, vehicle.id)
    }
}
