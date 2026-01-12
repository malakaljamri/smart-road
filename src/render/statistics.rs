use crate::render::sdl2_manager::Sdl2Manager;

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

    pub fn render_stats(&self, sdl2_manager: &mut Sdl2Manager, font: &sdl2::ttf::Font) {
        sdl2_manager.clear();

        let stats_lines = vec![
            format!("Max Vehicles Passed: {}", self.max_vehicles_passed),
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
