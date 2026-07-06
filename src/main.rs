use sdl2::event::Event;
use crate::winsdl::Winsdl;
mod winsdl;

fn main() {
    let mut winsdl = Winsdl::new(600, 400).expect("Failed to initialize SDL");

    println!("SDL initialized successfully!");

    'running: loop {
        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                _ => {}
            }
        }
    }
}
