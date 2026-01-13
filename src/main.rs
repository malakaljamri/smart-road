mod input;
mod intersection;
mod render;
mod types;

use crate::{
    input::InputHandler,
    render::{Statistics, TextureCache, draw_lanes},
    types::Direction,
};
use intersection::Lane;
use rand::Rng;
use render::{Sdl2Manager, Vehicle};
use sdl2::{image::InitFlag, pixels::Color, rect::Rect, render::TextureCreator};

/*
lane width = 35px
lane height = 295px
car width = 30px
*/

fn main() {
    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");

    let mut vehicles: Vec<Vehicle> = Vec::new();

    // Not fully used yet pending statistics implementation
    let mut statistics = Statistics::new();

    // Try to create the SDL2 manager (window size 800x600)
    let mut sdl2_manager = Sdl2Manager::new("Smart Road", 800, 800)
        .unwrap_or_else(|e| panic!("Failed to initialize SDL2: {}", e));

    let ttf_context = sdl2::ttf::init().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG).expect("Failed to initialize SDL_image");
    let font: sdl2::ttf::Font<'_, '_> =
        ttf_context.load_font("assets/fonts/Arial.ttf", 24).unwrap();

    let mut input = InputHandler::new();

    let mut event_pump = sdl2_manager
        .sdl_context
        .event_pump()
        .unwrap_or_else(|_| panic!("Failed to get SDL2 event pump"));

    // use texture creator to draw vehicles
    let texture_creator: TextureCreator<sdl2::video::WindowContext> =
        sdl2_manager.canvas.texture_creator();
    let texture_cache = TextureCache::new(&texture_creator);

    let mut rng = rand::thread_rng();
    let mut showing_stats = false;

    'running: loop {
        // Handle events (like closing the window)
        if !showing_stats {
            input.reset();
        }

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    break 'running;
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(keycode),
                    //? may need to add repeat flag check here to avoid spawning multiple vehicles on key hold
                    ..
                } => {
                    if showing_stats {
                        // If showing stats, ESC or any key exits
                        if keycode == sdl2::keyboard::Keycode::Escape {
                            break 'running;
                        }
                    } else {
                        input.handle_keydown(keycode);
                    }
                }
                _ => {}
            }
        }

        if input.quit && !showing_stats {
            showing_stats = true;
        }

        if showing_stats {
            // Render stats and wait for close
            statistics.render_stats(&mut sdl2_manager, &font);
            sdl2_manager.canvas.present();
            // Prevent high CPU usage while showing stats
            std::thread::sleep(std::time::Duration::from_millis(16));
            continue;
        }

        // south to north
        if input.spawn_south {
            let (random_dir, x) = match rng.gen_range(0..3) {
                0 => (Direction::East, 472.5),
                1 => (Direction::West, 402.5),
                _ => (Direction::North, 437.5),
            };

            let lane = Lane::set(Direction::South, random_dir);
            let vehicle = Vehicle::new(vehicles.len(), x, 800.0, lane);
            println!("vehicle: {:?}", vehicle);
            vehicles.push(vehicle);
        }

        // north to south
        if input.spawn_north {
            let (random_dir, x) = match rng.gen_range(0..3) {
                0 => (Direction::East, 365.0),
                1 => (Direction::West, 295.0),
                _ => (Direction::South, 330.0),
            };

            let lane = Lane::set(Direction::North, random_dir);
            let vehicle = Vehicle::new(vehicles.len(), x, 0.0, lane);
            println!("vehicle: {:?}", vehicle);
            vehicles.push(vehicle);
        }

        // east to west
        if input.spawn_east {
            let (random_dir, y) = match rng.gen_range(0..3) {
                0 => (Direction::North, 295.0),
                1 => (Direction::South, 365.0),
                _ => (Direction::West, 330.0),
            };

            let lane = Lane::set(Direction::East, random_dir);
            let vehicle = Vehicle::new(vehicles.len(), 800.0, y, lane);
            println!("vehicle: {:?}", vehicle);
            vehicles.push(vehicle);
        }

        // west to east
        if input.spawn_west {
            let (random_dir, y) = match rng.gen_range(0..3) {
                0 => (Direction::North, 402.5),
                1 => (Direction::South, 472.5),
                _ => (Direction::East, 437.5),
            };

            let lane = Lane::set(Direction::West, random_dir);
            let vehicle = Vehicle::new(vehicles.len(), 0.0, y, lane);
            println!("vehicle: {:?}", vehicle);
            vehicles.push(vehicle);
        }

        // Set the draw color to yellow and clear the window each frame
        sdl2_manager
            .canvas
            .set_draw_color(Color::RGB(255, 235, 59));
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

        draw_lanes(&mut sdl2_manager, &font, &texture_creator);

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
