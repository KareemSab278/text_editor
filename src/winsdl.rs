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
    let video_subsystem = sdl.video().unwrap();

    video_subsystem.text_input().start();

    let window = video_subsystem
        .window("Text Editor", width as u32, height as u32)
        .position_centered()
        .build()
        .unwrap();

    let canvas = window
        .into_canvas()
        .build()
        .unwrap();

    let event_pump = sdl.event_pump().unwrap();

    Ok(Self {
        sdl,
        canvas,
        event_pump,
    })
}
}