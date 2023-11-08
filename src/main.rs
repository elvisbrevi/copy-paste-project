use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use clipboard_files;

#[derive(PartialEq)]
enum ClipboardType {
    FilesOrDirectories,
    Text,
}

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut old_contents = String::from("");
    let mut new_contents = String::new();
    let mut clipboard_type;

    loop {
        // if clipboard is file or directory
        match clipboard_files::read() {
            Ok(file_paths) => {
                new_contents = String::new();
                clipboard_type = ClipboardType::FilesOrDirectories;
                // reading clipboard
                for file_path in file_paths {
                    match file_path.to_str() {
                        Some(value) => {
                            new_contents += value;
                        }
                        None => {}
                    }
                }
            }
            Err(_) => {
                clipboard_type = ClipboardType::Text;
            }
        }

        // if clipboard is only text
        if clipboard_type == ClipboardType::Text {
            match ctx.get_contents() {
                Ok(contents) => {
                    if contents != old_contents {
                        new_contents = contents.clone();
                    }
                }
                Err(_) => {}
            }
        }

        // action with clipboard
        if old_contents != new_contents {
            match clipboard_type {
                ClipboardType::FilesOrDirectories => println!("archivo o directorio:"),
                ClipboardType::Text => println!("texto:"),
            }
            println!("old: {} \nnew: {}", old_contents, new_contents);
            println!("---------------------------------------------");
        }
        old_contents = new_contents.clone();
    }
}
