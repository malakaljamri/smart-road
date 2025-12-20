mod render;
use render::Sdl2Manager;

fn main() {
    // Try to create the SDL2 manager (window size 800x600)
    let mut sdl2_manager = match Sdl2Manager::new("Smart Road", 800, 600) {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("Failed to initialize SDL2: {}", e);
            return;
        }
    };

    // Get the SDL2 event pump to handle window events
    let mut event_pump = sdl2_manager
        .sdl_context
        .event_pump()
        .expect("Failed to get event pump");

    'running: loop {
        // Handle events (like closing the window)
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        // Set the draw color to yellow and clear the window each frame
        sdl2_manager
            .canvas
            .set_draw_color(sdl2::pixels::Color::RGB(255, 235, 59));
        sdl2_manager.canvas.clear();
        sdl2_manager.canvas.present();
    }
}
