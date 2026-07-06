use sdl2::{render::Canvas, video::Window, Sdl, rect::Rect};

pub struct Winsdl {
    pub sdl: Sdl,
    pub canvas: Canvas<Window>,
    pub event_pump: sdl2::EventPump,
}

impl Winsdl {
    pub fn new(width: usize, height: usize) -> Result<Self, &'static str> {
        let sdl: Sdl = sdl2::init().expect("Failed to initialize SDL");
        let video_subsystem = sdl.video().unwrap();

        let window = video_subsystem
            .window("Text Editor", width as u32, height as u32)
            .position_centered()
            .build()
            .expect("Failed to create window");

        let mut canvas = window
            .into_canvas()
            .build()
            .expect("Failed to create canvas");

        canvas.set_draw_color(sdl2::pixels::Color::RGB(222, 222, 200));
        canvas.clear();
        let _ = canvas.fill_rect(Rect::new(10, 10, 780, 580));
        canvas.present();

        let event_pump = sdl.event_pump().unwrap();

        Ok(Winsdl {
            sdl,
            canvas,
            event_pump,
        })
    }
}