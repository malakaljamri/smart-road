mod input;
mod render;
mod traffic;
mod types;

use crate::{
    input::InputHandler,
    render::{Statistics, TextureCache, draw_roads},
};
use render::{Sdl2Manager, Vehicle};
use sdl2::{image::InitFlag, render::TextureCreator};
use traffic::traffic_manager;

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
    let texture_cache: TextureCache<'_> = TextureCache::new(&texture_creator);

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

        draw_roads(&mut sdl2_manager, &font, &texture_creator);

        traffic_manager(&mut input, &mut vehicles);

        Vehicle::render(&vehicles, &texture_cache, &mut sdl2_manager);

        sdl2_manager.canvas.present();
    }
}
