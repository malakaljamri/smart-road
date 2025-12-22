mod input;
mod render;
mod sim;

use crate::input::InputHandler;

use render::Sdl2Manager;
use sdl2::{
    image::{InitFlag, LoadTexture},
    rect::Rect,
};

fn main() {
    // Try to create the SDL2 manager (window size 800x600)
    let mut sdl2_manager = match Sdl2Manager::new("Smart Road", 800, 600) {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("Failed to initialize SDL2: {}", e);
            return;
        }
    };

    let mut input = InputHandler::new();

    // Get the SDL2 event pump to handle window events
    let mut event_pump = sdl2_manager
        .sdl_context
        .event_pump()
        .expect("Failed to get event pump");

    'running: loop {
        // Handle events (like closing the window)

        input.reset();

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    input.quit = true;
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    input.handle_keydown(keycode);
                }
                _ => {}
            }
        }

        if input.quit {
            break 'running;
        }

        // Set the draw color to yellow and clear the window each frame
        sdl2_manager
            .canvas
            .set_draw_color(sdl2::pixels::Color::RGB(255, 235, 59));
        sdl2_manager.canvas.clear();

        let _image_context = sdl2::image::init(InitFlag::PNG).unwrap();

        // Create a texture from the vehicle PNG
        let texture_creator = sdl2_manager.canvas.texture_creator();
        let vehicle_texture = texture_creator
            .load_texture("assets/vehicles/east/car_24px_blue_3.png")
            .expect("Failed to load vehicle PNG");

        // ...inside your main loop, after sdl2_manager.canvas.clear(); and before present():
        sdl2_manager
            .canvas
            .copy(&vehicle_texture, None, Some(Rect::new(100, 100, 24, 24)))
            .unwrap();

        sdl2_manager.canvas.present();
    }
}
