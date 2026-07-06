use sdl2::{ event::Event, keyboard::Keycode, pixels::Color };

mod winsdl;
mod editor;

use winsdl::Winsdl;
use editor::Editor;

fn main() {
    let mut winsdl = Winsdl::new(600, 400).expect("SDL failed");

    let mut editor = Editor::new();

    winsdl.sdl.video().expect("error loading video subsystem").text_input().start();

    let ttf_context = sdl2::ttf::init().expect("Failed to initialize TTF context");

    let font = ttf_context.load_font("fonts/DejaVuSans.ttf", 18).expect("Failed to load font");

    'running: loop {
        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }

                Event::TextInput { text, .. } => {
                    for c in text.chars() {
                        editor.insert(c);
                    }
                }

                Event::KeyDown { keycode: Some(Keycode::Backspace), .. } => {
                    editor.backspace();
                }

                Event::KeyDown { keycode: Some(Keycode::S), keymod, .. } => {
                    if keymod.contains(sdl2::keyboard::Mod::LCTRLMOD) {
                        editor.save("test.txt");
                    }
                }

                _ => {}
            }
        }

        winsdl.canvas.set_draw_color(Color::RGB(30, 30, 30));
        winsdl.canvas.clear();

        if !editor.text.is_empty() {
            let surface = font.render(&editor.text).blended(Color::RGB(220, 220, 220)).unwrap();

            let texture_creator = winsdl.canvas.texture_creator();

            let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

            winsdl.canvas
                .copy(
                    &texture,
                    None,
                    Some(sdl2::rect::Rect::new(10, 10, surface.width(), surface.height()))
                )
                .unwrap();
        }

        winsdl.canvas.present();
    }
}
