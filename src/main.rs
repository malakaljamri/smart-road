mod input;
mod render;
mod sim;

use crate::input::InputHandler;

use render::Sdl2Manager;
use sdl2::{
    image::{InitFlag, LoadTexture}, pixels::Color, rect::{Point, Rect}
};

fn main() {
    // Try to create the SDL2 manager (window size 800x600)
    let mut sdl2_manager = match Sdl2Manager::new("Smart Road", 800, 800) {
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

        // corner rects
        sdl2_manager.canvas.set_draw_color(Color::RGB(0, 0, 0));
        let top_left = Rect::new(0, 0, 295, 295);
        let top_right = Rect::new(505, 0, 295, 295);
        let bottom_left = Rect::new(0, 505, 295, 295);
        let bottom_right = Rect::new(505, 505, 295, 295);

        // draw corner rects
        sdl2_manager.canvas.fill_rect(top_left).unwrap();
        sdl2_manager.canvas.fill_rect(top_right).unwrap();
        sdl2_manager.canvas.fill_rect(bottom_left).unwrap();
        sdl2_manager.canvas.fill_rect(bottom_right).unwrap();

        // draw lanes
        // top
        sdl2_manager.canvas.draw_line(Point::new(330, 0), Point::new(330, 295));
        sdl2_manager.canvas.draw_line(Point::new(365, 0), Point::new(365, 295));
        sdl2_manager.canvas.draw_line(Point::new(400, 0), Point::new(400, 295));
        sdl2_manager.canvas.draw_line(Point::new(435, 0), Point::new(435, 295));
        sdl2_manager.canvas.draw_line(Point::new(470, 0), Point::new(470, 295));
        sdl2_manager.canvas.draw_line(Point::new(295, 295), Point::new(400, 295)); // stop

        // bottom
        sdl2_manager.canvas.draw_line(Point::new(330, 505), Point::new(330, 800));
        sdl2_manager.canvas.draw_line(Point::new(365, 505), Point::new(365, 800));
        sdl2_manager.canvas.draw_line(Point::new(400, 505), Point::new(400, 800));
        sdl2_manager.canvas.draw_line(Point::new(435, 505), Point::new(435, 800));
        sdl2_manager.canvas.draw_line(Point::new(470, 505), Point::new(470, 800));
        sdl2_manager.canvas.draw_line(Point::new(400, 505), Point::new(505, 505)); // stop

        // left
        sdl2_manager.canvas.draw_line(Point::new(0, 330), Point::new(295, 330));
        sdl2_manager.canvas.draw_line(Point::new(0, 365), Point::new(295, 365));
        sdl2_manager.canvas.draw_line(Point::new(0, 400), Point::new(295, 400));
        sdl2_manager.canvas.draw_line(Point::new(0, 435), Point::new(295, 435));
        sdl2_manager.canvas.draw_line(Point::new(0, 470), Point::new(295, 470));
        sdl2_manager.canvas.draw_line(Point::new(295, 400), Point::new(295, 505)); // stop

        // right
        sdl2_manager.canvas.draw_line(Point::new(505, 330), Point::new(800, 330));
        sdl2_manager.canvas.draw_line(Point::new(505, 365), Point::new(800, 365));
        sdl2_manager.canvas.draw_line(Point::new(505, 400), Point::new(800, 400));
        sdl2_manager.canvas.draw_line(Point::new(505, 435), Point::new(800, 435));
        sdl2_manager.canvas.draw_line(Point::new(505, 470), Point::new(800, 470));
        sdl2_manager.canvas.draw_line(Point::new(505, 295), Point::new(505, 400)); // stop

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
            Some(Rect::new(100, 402, 30, 30)),
        ) {
            Ok(_) => {}
            Err(e) => eprintln!("Could not copy texture to canvas: {}", e),
        };

        sdl2_manager.canvas.present();
    }
}
