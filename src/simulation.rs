pub struct Simulation {
    pub next_vehicle_id: usize,
}

impl Simulation {
    pub fn new() -> Self {
        Simulation {
            next_vehicle_id: 0,
        }
    }

    pub fn get_next_vehicle_id(&mut self) -> usize {
        let id = self.next_vehicle_id;
        self.next_vehicle_id += 1;
        id
    }
}
