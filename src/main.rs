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
                new_contents.clear();
                clipboard_type = ClipboardType::FilesOrDirectories;
                // reading clipboard
                for file_path in file_paths {
                    if let Some(value) = file_path.to_str() {
                        new_contents.push_str(value)
                    }
                }
            }
            Err(_) => {
                clipboard_type = ClipboardType::Text;
            }
        }

        // if clipboard is only text
        if clipboard_type == ClipboardType::Text {
            if let Ok(contents) = ctx.get_contents() {
                if contents != old_contents {
                    new_contents = contents;
                }
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
        //old_contents = std::mem::take(&mut new_contents);
        old_contents = &new_contents;
    }
}
