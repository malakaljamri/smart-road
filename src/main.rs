mod input;
mod intersection;
mod render;
mod types;

use crate::{input::InputHandler, render::TextureCache, types::Direction};
use intersection::Lane;
use rand::Rng;
use render::{Sdl2Manager, Vehicle};
use sdl2::{
    image::InitFlag,
    pixels::Color,
    rect::{Point, Rect},
};

// lane width = 35px
// lane height = 295px
// car width = 30px

fn main() {
    let mut vehicles: Vec<Vehicle> = Vec::new();

    // Try to create the SDL2 manager (window size 800x600)
    let mut sdl2_manager = Sdl2Manager::new("Smart Road", 800, 800)
        .unwrap_or_else(|e| panic!("Failed to initialize SDL2: {}", e));

    let ttf_context = sdl2::ttf::init().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG).expect("Failed to initialize SDL_image");
    let font = ttf_context.load_font("assets/fonts/Arial.ttf", 24).unwrap();

    let mut input = InputHandler::new();

    let mut event_pump = sdl2_manager
        .sdl_context
        .event_pump()
        .unwrap_or_else(|_| panic!("Failed to get SDL2 event pump"));

    // use texture creator to draw vehicles
    let texture_creator = sdl2_manager.canvas.texture_creator();
    let texture_cache = TextureCache::new(&texture_creator);

    let mut rng = rand::thread_rng();

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
                    //? may need to add repeat flag check here to avoid spawning multiple vehicles on key hold
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

        // south to north
        if input.spawn_south {
            let (random_dir, x) = match rng.gen_range(0..3) {
                0 => (Direction::East, 472.5),
                1 => (Direction::West, 402.5),
                _ => (Direction::North, 437.5),
            };
            // println!("Spawn vehicle from south to {:?}", random_dir);

            let lane = Lane::set(types::Direction::South, random_dir);
            let vehicle = Vehicle::new(vehicles.len(), x, 800.0, lane);
            println!("vehicle: {:?}", vehicle);
            vehicles.push(vehicle);
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

        //* Font and text */
        let text_surface = font.render("Smart Road").blended(Color::BLACK).unwrap();
        let texture_creator = sdl2_manager.canvas.texture_creator();
        let text_texture = texture_creator
            .create_texture_from_surface(&text_surface)
            .unwrap();

        let target = Rect::new(
            800 / 2 - (text_surface.width() as i32) / 2,
            400 - (text_surface.height() as i32) / 2,
            text_surface.width(),
            text_surface.height(),
        );
        sdl2_manager
            .canvas
            .copy(&text_texture, None, Some(target))
            .unwrap();

        // draw lanes
        // top
        sdl2_manager
            .canvas
            .draw_line(Point::new(330, 0), Point::new(330, 295))
            .unwrap();

        //* write "r" on the road to indicate stop line
        let r_surface = font.render("r").blended(Color::BLACK).unwrap();
        let r_texture = texture_creator
            .create_texture_from_surface(&r_surface)
            .unwrap();

        let r_target = Rect::new(305, 270, r_surface.width(), r_surface.height());

        sdl2_manager
            .canvas
            .copy(&r_texture, None, Some(r_target))
            .unwrap();

        //* end of "r" drawing

        sdl2_manager
            .canvas
            .draw_line(Point::new(365, 0), Point::new(365, 295))
            .unwrap();

        //* write "s" on the road to indicate stop line
        let s_surface = font.render("s").blended(Color::BLACK).unwrap();
        let s_texture = texture_creator
            .create_texture_from_surface(&s_surface)
            .unwrap();

        let s_target = Rect::new(340, 270, s_surface.width(), s_surface.height());
        sdl2_manager
            .canvas
            .copy(&s_texture, None, Some(s_target))
            .unwrap();

        //* end of "s" drawing

        sdl2_manager
            .canvas
            .draw_line(Point::new(400, 0), Point::new(400, 295))
            .unwrap();

        //* write "l" on the road to indicate stop line
        let l_surface = font.render("l").blended(Color::BLACK).unwrap();
        let l_texture = texture_creator
            .create_texture_from_surface(&l_surface)
            .unwrap();
        let l_target = Rect::new(380, 270, l_surface.width(), l_surface.height());

        sdl2_manager
            .canvas
            .copy(&l_texture, None, Some(l_target))
            .unwrap();

        //* end of "l" drawing

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

        //* write "r" on the road to indicate stop line
        let r_surface = font.render("r").blended(Color::BLACK).unwrap();
        let r_texture = texture_creator
            .create_texture_from_surface(&r_surface)
            .unwrap();
        let r_target = Rect::new(485, 505, r_surface.width(), r_surface.height());
        sdl2_manager
            .canvas
            .copy(&r_texture, None, Some(r_target))
            .unwrap();

        //* write "s" on the road to indicate stop line
        let s_surface = font.render("s").blended(Color::BLACK).unwrap();
        let s_texture = texture_creator
            .create_texture_from_surface(&s_surface)
            .unwrap();
        let s_target = Rect::new(448, 505, s_surface.width(), s_surface.height());
        sdl2_manager
            .canvas
            .copy(&s_texture, None, Some(s_target))
            .unwrap();

        //* end of "s" drawing

        sdl2_manager
            .canvas
            .draw_line(Point::new(470, 505), Point::new(470, 800))
            .unwrap();

        //* write "l" on the road to indicate stop line
        let l_surface = font.render("l").blended(Color::BLACK).unwrap();
        let l_texture = texture_creator
            .create_texture_from_surface(&l_surface)
            .unwrap();
        let l_target = Rect::new(415, 505, l_surface.width(), l_surface.height());

        sdl2_manager
            .canvas
            .copy(&l_texture, None, Some(l_target))
            .unwrap();
        //* end of "l" drawing

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

        //* write "l" on the road to indicate stop line
        let l_surface = font.render("l").blended(Color::BLACK).unwrap();
        let l_texture = texture_creator
            .create_texture_from_surface(&l_surface)
            .unwrap();
        let l_target = Rect::new(280, 403, l_surface.width(), l_surface.height());
        sdl2_manager
            .canvas
            .copy_ex(&l_texture, None, Some(l_target), 90.0, None, false, false)
            .unwrap();
        //* end of "l" drawing

        sdl2_manager
            .canvas
            .draw_line(Point::new(0, 470), Point::new(295, 470))
            .unwrap();

        //* write "s" on the road to indicate stop line
        let s_surface = font.render("s").blended(Color::BLACK).unwrap();
        let s_texture = texture_creator
            .create_texture_from_surface(&s_surface)
            .unwrap();
        let s_target = Rect::new(275, 438, s_surface.width(), s_surface.height());

        sdl2_manager
            .canvas
            .copy_ex(&s_texture, None, Some(s_target), 90.0, None, false, false)
            .unwrap();
        //* end of "s" drawing

        //* write "r" on the road to indicate stop line
        let r_surface = font.render("r").blended(Color::BLACK).unwrap();
        let r_texture = texture_creator
            .create_texture_from_surface(&r_surface)
            .unwrap();
        let r_target = Rect::new(275, 473, r_surface.width(), r_surface.height());

        sdl2_manager
            .canvas
            .copy_ex(&r_texture, None, Some(r_target), 90.0, None, false, false)
            .unwrap();
        //* end of "r" drawing

        sdl2_manager
            .canvas
            .draw_line(Point::new(295, 400), Point::new(295, 505)) // stop
            .unwrap();

        // right
        sdl2_manager
            .canvas
            .draw_line(Point::new(505, 330), Point::new(800, 330))
            .unwrap();

        //* write "r" on the road to indicate stop line
        let r_surface = font.render("r").blended(Color::BLACK).unwrap();
        let r_texture = texture_creator
            .create_texture_from_surface(&r_surface)
            .unwrap();
        let r_target = Rect::new(510, 300, r_surface.width(), r_surface.height());

        sdl2_manager
            .canvas
            .copy_ex(&r_texture, None, Some(r_target), 270.0, None, false, false)
            .unwrap();
        //* end of "r" drawing

        sdl2_manager
            .canvas
            .draw_line(Point::new(505, 365), Point::new(800, 365))
            .unwrap();

        //* write "s" on the road to indicate stop line
        let s_surface = font.render("s").blended(Color::BLACK).unwrap();
        let s_texture = texture_creator
            .create_texture_from_surface(&s_surface)
            .unwrap();
        let s_target = Rect::new(510, 335, s_surface.width(), s_surface.height());
        sdl2_manager
            .canvas
            .copy_ex(&s_texture, None, Some(s_target), 270.0, None, false, false)
            .unwrap();
        //* end of "s" drawing

        sdl2_manager
            .canvas
            .draw_line(Point::new(505, 400), Point::new(800, 400))
            .unwrap();

        //* write "l" on the road to indicate stop line
        let l_surface = font.render("l").blended(Color::BLACK).unwrap();
        let l_texture = texture_creator
            .create_texture_from_surface(&l_surface)
            .unwrap();
        let l_target = Rect::new(515, 370, l_surface.width(), l_surface.height());

        sdl2_manager
            .canvas
            .copy_ex(&l_texture, None, Some(l_target), 270.0, None, false, false)
            .unwrap();
        //* end of "l" drawing

        sdl2_manager
            .canvas
            .draw_line(Point::new(505, 435), Point::new(800, 435))
            .unwrap(); // stop
        sdl2_manager
            .canvas
            .draw_line(Point::new(505, 470), Point::new(800, 470))
            .unwrap();
        sdl2_manager
            .canvas
            .draw_line(Point::new(505, 295), Point::new(505, 400)) // stop
            .unwrap();

        // update vehicles
        for vehicle in &mut vehicles {
            vehicle.update();
        }

        for vehicle in &vehicles {
            let vehicle_texture = texture_cache.get(vehicle.color, vehicle.direction);

            sdl2_manager
                .canvas
                .copy(
                    &vehicle_texture,
                    None,
                    Some(Rect::new(vehicle.x as i32, vehicle.y as i32, 30, 30)),
                )
                .unwrap();
        }

        sdl2_manager.canvas.present();
    }
}
