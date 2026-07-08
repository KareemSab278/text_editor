use rfd::FileDialog;

pub fn save_file(content: &str) -> () {
    let file_path = FileDialog::new()
        .set_title("Save your file")
        .add_filter("text", &["txt"])
        .add_filter("rust", &["rs"])
        .save_file();

    if let Some(path) = file_path {
        println!("File will be saved to: {:?}", path);
        std::fs::write(path, content).expect("Failed to save file");
    } else {
        println!("User cancelled the dialog.");
    }
}


pub fn open_file() -> Option<String> {
    let file_path = FileDialog::new()
        .set_title("Open a file")
        .add_filter("text", &["txt"])
        .add_filter("rust", &["rs"])
        .pick_file();

    if let Some(path) = file_path {
        println!("File will be opened from: {:?}", path);
        match std::fs::read_to_string(&path) {
            Ok(content) => Some(content),
            Err(e) => {
                eprintln!("Failed to read file: {}", e);
                None
            }
        }
    } else {
        println!("User cancelled the dialog.");
        None
    }
}