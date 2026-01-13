use sdl2::Sdl;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Sdl2Manager {
    pub sdl_context: Sdl,
    pub canvas: Canvas<Window>,
}

impl Sdl2Manager {
    /**
     * Initializes SDL2, creates a window and a canvas for rendering.
     *
     * # Arguments
     *
     * * `title` - The title of the window.
     * * `width` - The width of the window.
     * * `height` - The height of the window.
     *
     * # Returns
     *
     * A Result containing the initialized Sdl2Manager or an error string.
     */
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, String> {
        // Initialize SDL2
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok(Sdl2Manager {
            sdl_context,
            canvas,
        })
    }

    /**
     * Clears the canvas with a black color.
     * This should be called at the beginning of each frame before drawing.
     */
    pub fn clear(&mut self) {
        self.canvas
            .set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    // /**
    //  * Presents the current canvas content to the window.
    //  * This should be called after all drawing operations are complete.
    //  */
    // pub fn present(&mut self) {
    //     self.canvas.present();
    // }
}
