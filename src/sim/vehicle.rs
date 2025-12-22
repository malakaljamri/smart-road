use crate::render::WorldPos;
use crate::sim::types::{LaneId, VehicleState, VelocityLevel};

pub struct Vehicle {
    pub id: u64,
    pub lane: LaneId,
    pub state: VehicleState,

    pub position: WorldPos,
    pub rotation: f32,

    pub velocity: VelocityLevel,
    pub distance_remainig: f32,
    pub time_in_intersection: f32,
}
