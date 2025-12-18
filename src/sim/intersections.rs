use crate::sim::{types::*, vehicle::Vehicle};

pub struct Intersection {
    pub vehicles: Vec<Vehicle>,
    pub safety_distance: f32,
    pub controller: IntersectionController,
}

pub struct IntersectionController {
    active_lanes:   Vec<LaneId>,
}