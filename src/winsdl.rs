use sdl2::{
    render::Canvas,
    video::Window,
    Sdl,
};

pub struct Winsdl {
    pub sdl: Sdl,
    pub canvas: Canvas<Window>,
    pub event_pump: sdl2::EventPump,
}

impl Winsdl {
    pub fn new(width: usize, height: usize) -> Result<Self, &'static str> {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();

        let window = video
            .window("Text Editor", width as u32, height as u32)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(
            sdl2::pixels::Color::RGB(30, 30, 30)
        );

        canvas.clear();
        canvas.present();

        let event_pump = sdl.event_pump().unwrap();

        Ok(Self {
            sdl,
            canvas,
            event_pump,
        })
    }
}