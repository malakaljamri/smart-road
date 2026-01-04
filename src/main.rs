mod input;
mod render;
mod types;

use crate::input::InputHandler;

use render::Sdl2Manager;
use sdl2::{
    image::{InitFlag, LoadTexture},
    pixels::Color,
    rect::{Point, Rect},
};

fn main() {
    // Try to create the SDL2 manager (window size 800x600)
    let mut sdl2_manager = Sdl2Manager::new("Smart Road", 800, 800)
        .unwrap_or_else(|e| panic!("Failed to initialize SDL2: {}", e));

    let mut input = InputHandler::new();

    let mut event_pump = sdl2_manager
        .sdl_context
        .event_pump()
        .unwrap_or_else(|_| panic!("Failed to get SDL2 event pump"));

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
        sdl2_manager
            .canvas
            .draw_line(Point::new(330, 0), Point::new(330, 295))
            .unwrap();
        sdl2_manager
            .canvas
            .draw_line(Point::new(365, 0), Point::new(365, 295))
            .unwrap();
        sdl2_manager
            .canvas
            .draw_line(Point::new(400, 0), Point::new(400, 295))
            .unwrap();
        sdl2_manager
            .canvas
            .draw_line(Point::new(435, 0), Point::new(435, 295))
            .unwrap();
        sdl2_manager
            .canvas
            .draw_line(Point::new(470, 0), Point::new(470, 295))
            .unwrap();
        sdl2_manager
            .canvas
            .draw_line(Point::new(295, 295), Point::new(400, 295)) // stop
            .unwrap();

        // bottom
        sdl2_manager
            .canvas
            .draw_line(Point::new(330, 505), Point::new(330, 800))
            .unwrap();
        sdl2_manager
            .canvas
            .draw_line(Point::new(365, 505), Point::new(365, 800))
            .unwrap();
        sdl2_manager
            .canvas
            .draw_line(Point::new(400, 505), Point::new(400, 800))
            .unwrap();
        sdl2_manager
            .canvas
            .draw_line(Point::new(435, 505), Point::new(435, 800))
            .unwrap();
        sdl2_manager
            .canvas
            .draw_line(Point::new(470, 505), Point::new(470, 800))
            .unwrap();
        sdl2_manager
            .canvas
            .draw_line(Point::new(400, 505), Point::new(505, 505)) // stop
            .unwrap();

        // left
        sdl2_manager
            .canvas
            .draw_line(Point::new(0, 330), Point::new(295, 330))
            .unwrap();
        sdl2_manager
            .canvas
            .draw_line(Point::new(0, 365), Point::new(295, 365))
            .unwrap();
        sdl2_manager
            .canvas
            .draw_line(Point::new(0, 400), Point::new(295, 400))
            .unwrap();
        sdl2_manager
            .canvas
            .draw_line(Point::new(0, 435), Point::new(295, 435))
            .unwrap();
        sdl2_manager
            .canvas
            .draw_line(Point::new(0, 470), Point::new(295, 470))
            .unwrap();
        sdl2_manager
            .canvas
            .draw_line(Point::new(295, 400), Point::new(295, 505)) // stop
            .unwrap();

        // right
        sdl2_manager
            .canvas
            .draw_line(Point::new(505, 330), Point::new(800, 330))
            .unwrap();
        sdl2_manager
            .canvas
            .draw_line(Point::new(505, 365), Point::new(800, 365))
            .unwrap();
        sdl2_manager
            .canvas
            .draw_line(Point::new(505, 400), Point::new(800, 400))
            .unwrap();
        sdl2_manager
            .canvas
            .draw_line(Point::new(505, 435), Point::new(800, 435))
            .unwrap();
        sdl2_manager
            .canvas
            .draw_line(Point::new(505, 470), Point::new(800, 470))
            .unwrap();
        sdl2_manager
            .canvas
            .draw_line(Point::new(505, 295), Point::new(505, 400)) // stop
            .unwrap();

        let _image_context = sdl2::image::init(InitFlag::PNG).unwrap();

        // Create a texture from the vehicle PNG
        let texture_creator = sdl2_manager.canvas.texture_creator();

        let vehicle_texture = texture_creator
            .load_texture("assets/vehicles/east/car_24px_blue_3.png")
            .unwrap();

        // ...inside your main loop, after sdl2_manager.canvas.clear(); and before present():
        sdl2_manager
            .canvas
            .copy(&vehicle_texture, None, Some(Rect::new(100, 402, 30, 30)))
            .unwrap();

        sdl2_manager.canvas.present();
    }
}
