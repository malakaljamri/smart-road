mod input;
mod render;
mod sim;

use crate::input::InputHandler;

use render::Sdl2Manager;
use sdl2::{
    event,
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

    let mut event_pump = match sdl2_manager.sdl_context.event_pump() {
        Ok(pump) => pump,
        Err(e) => {
            eprintln!("Failed to get event pump: {}", e);
            return;
        }
    };

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
        let vehicle_texture =
            match texture_creator.load_texture("assets/vehicles/east/car_24px_blue_3.png") {
                Ok(texture) => texture,
                Err(e) => {
                    eprintln!("Could not load vehicle PNG: {}", e);
                    // Optionally: return, use a default texture, etc.
                    continue; // skips drawing this frame
                }
            };

        // ...inside your main loop, after sdl2_manager.canvas.clear(); and before present():
        let _ = match sdl2_manager.canvas.copy(
            &vehicle_texture,
            None,
            Some(Rect::new(100, 100, 24, 24)),
        ) {
            Ok(_) => {}
            Err(e) => eprintln!("Could not copy texture to canvas: {}", e),
        };

        sdl2_manager.canvas.present();
    }
}
