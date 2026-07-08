use sdl2::clipboard;

use crate::file;

pub struct Editor {
    pub text: String,
    pub cursor: usize,
    pub selected_range: Option<(usize, usize)>,
}

pub trait EditorTrait {
    // so this is like interface in ts. and Editor is just the struct that implements the interface. i do this all the time in ts lol
    fn new() -> Self;
    fn insert(&mut self, c: char);
    fn backspace(&mut self);
    fn save(&self);
    fn open(&mut self);
    fn select(&mut self, start: usize, end: usize);
    fn copy(&self, clipboard: &clipboard::ClipboardUtil);
    fn paste(&mut self, clipboard: &clipboard::ClipboardUtil);
}

impl EditorTrait for Editor {
    fn new() -> Self {
        Self {
            text: String::new(),
            cursor: 0,
            selected_range: None,
        }
    }

    fn insert(&mut self, c: char) {
        self.text.insert(self.cursor, c);
        self.cursor += c.len_utf8();
    }

    fn backspace(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.text.remove(self.cursor);
        }
    }

    fn save(&self) {
        file::save_file(&self.text);
    }

    fn open(&mut self) {
        if let Some(content) = file::open_file() {
            self.text = content;
            self.cursor = self.text.len();
        }
    }

    fn select(&mut self, start: usize, end: usize) {
        if self.cursor >= start && self.cursor <= end {
            self.selected_range = Some((start, end));
            // needs more logic to handle selection, but this is a start
        }
    }

    fn copy(&self, clipboard: &clipboard::ClipboardUtil) {
        clipboard.set_clipboard_text(&self.text).expect("Failed to copy text to clipboard");
    }

    fn paste(&mut self, clipboard: &clipboard::ClipboardUtil) {
        if let Ok(text) = clipboard.clipboard_text() {
            self.text.insert_str(self.cursor, &text);
            self.cursor += text.len();
        }
    }
}
