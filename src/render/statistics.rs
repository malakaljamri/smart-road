use crate::render::sdl2_manager::Sdl2Manager;
use crate::render::vehicle::Vehicle;

pub struct Statistics {
    pub max_vehicles_passed: usize,
    pub max_velocity: f32,
    pub min_velocity: f32,
    pub max_crossing_time: f32,
    pub min_crossing_time: f32,
    pub close_calls: usize,
}

impl Statistics {
    pub fn new() -> Self {
        Statistics {
            max_vehicles_passed: 0,
            max_velocity: 0.0,
            min_velocity: f32::MAX,
            max_crossing_time: 0.0,
            min_crossing_time: f32::MAX,
            close_calls: 0,
        }
    }

    pub fn calculate_intersection_stats(&mut self, completed_vehicles: &[Vehicle]) {
        if completed_vehicles.is_empty() {
            return;
        }
        
        // Max vehicles that passed the intersection
        self.max_vehicles_passed = completed_vehicles.len();
        
        // Calculate velocity stats from all vehicles
        let max_speeds: Vec<f32> = completed_vehicles.iter().map(|v| v.max_speed_reached).collect();
        let min_speeds: Vec<f32> = completed_vehicles.iter().map(|v| v.min_speed_reached).collect();
        
        if !max_speeds.is_empty() {
            self.max_velocity = max_speeds.iter().fold(0.0f32, |a, &b| a.max(b));
        }
        if !min_speeds.is_empty() {
            self.min_velocity = min_speeds.iter().fold(f32::MAX, |a, &b| a.min(b));
        }
        
        // Calculate crossing time stats (only vehicles that completed intersection)
        let crossing_times: Vec<f32> = completed_vehicles.iter()
            .filter_map(|v| v.intersection_entry_time)
            .collect();
            
        if !crossing_times.is_empty() {
            self.max_crossing_time = crossing_times.iter().fold(0.0f32, |a, &b| a.max(b));
            self.min_crossing_time = crossing_times.iter().fold(f32::MAX, |a, &b| a.min(b));
        }
        
        // Count close calls
        self.close_calls = completed_vehicles.iter().filter(|v| v.had_close_call).count();
    }

    pub fn calculate_simple_stats(&mut self, vehicles: &[Vehicle]) {
        if !vehicles.is_empty() {
            // Calculate velocity stats from current speeds
            let speeds: Vec<f32> = vehicles.iter().map(|v| v.speed).collect();
            self.max_velocity = speeds.iter().fold(0.0f32, |a, &b| a.max(b));
            self.min_velocity = speeds.iter().fold(f32::MAX, |a, &b| a.min(b));
            
            // Count vehicles (simple count)
            self.max_vehicles_passed = vehicles.len();
            
            // Simple close call detection (vehicles within safe distance)
            self.close_calls = 0;
            for (i, v1) in vehicles.iter().enumerate() {
                for v2 in vehicles.iter().skip(i + 1) {
                    let distance = ((v1.x - v2.x).powi(2) + (v1.y - v2.y).powi(2)).sqrt();
                    if distance < v1.collision.safe_distance {
                        self.close_calls += 1;
                        break;
                    }
                }
            }
        }
    }

    pub fn render_stats(&self, sdl2_manager: &mut Sdl2Manager, font: &sdl2::ttf::Font) {
        sdl2_manager.clear();

        let stats_lines = vec![
            format!("Max Vehicles Passed the intersection: {}", self.max_vehicles_passed),
            format!("Max Velocity: {:.2}", self.max_velocity),
            format!("Min Velocity: {:.2}", self.min_velocity),
            format!("Max Crossing Time: {:.2}", self.max_crossing_time),
            format!("Min Crossing Time: {:.2}", self.min_crossing_time),
            format!("Close Calls: {}", self.close_calls),
            format!("Press ESC to Exit"),
        ];

        let texture_creator = sdl2_manager.canvas.texture_creator();
        let line_height = 30;

        for (i, line) in stats_lines.iter().enumerate() {
            let surface = font
                .render(line)
                .blended(sdl2::pixels::Color::WHITE)
                .unwrap();
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .unwrap();
            let y_pos = 10 + (i as i32 * line_height);
            let target = sdl2::rect::Rect::new(10, y_pos, surface.width(), surface.height());

            sdl2_manager
                .canvas
                .copy(&texture, None, Some(target))
                .unwrap();
        }
    }
}
