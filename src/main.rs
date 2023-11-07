// extern crate clipboard;

// use clipboard::ClipboardContext;
// use clipboard::ClipboardProvider;
// use std::{thread::sleep, time::Duration};

// fn main() {
//     let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
//     let mut last_contents = String::new();

//     loop {
//         match ctx.get_contents() {
//             Ok(contents) => {
//                 if contents != last_contents {
//                     println!("Nuevo contenido detectado en el portapapeles! {}", contents);
//                     last_contents = contents.clone();
//                     // Intenta interpretar el contenido como una ruta de archivo
//                     if let Ok(metadata) = std::fs::metadata(&contents) {
//                         if metadata.is_file() {
//                             println!("El contenido parece ser una ruta de archivo: {}", contents);
//                         } else if metadata.is_dir() {
//                             println!(
//                                 "El contenido parece ser una ruta de directorio: {}",
//                                 contents
//                             );
//                         }
//                     } else {
//                         println!("solo texto");
//                     }
//                 }
//             }
//             Err(e) => println!("Error al acceder al portapapeles: {:?}", e),
//         }

//         sleep(Duration::from_secs(2));
//     }
// }

use clipboard_files;

fn main() {
    let mut last_contents = String::from("");

    loop {
        match clipboard_files::read() {
            Ok(file_paths) => {
                let mut new_contents = String::new();
                // reading clipboard
                for file_path in file_paths {
                    match file_path.to_str() {
                        Some(value) => {
                            if file_path.is_file() {
                                //println!("Files: {}", value);
                            }
                            if file_path.is_dir() {
                                //println!("Dir: {}", value);
                            }

                            new_contents += value;
                        }
                        None => {
                            new_contents = "nada".to_string();
                        }
                    }
                }

                if last_contents != new_contents {
                    println!("old: {} \nnew: {}", last_contents, new_contents);
                    println!("---------------------------------------------");
                }
                last_contents = new_contents.clone();
            }
            Err(e) => {
                //new_contents = "error".to_string();
            }
        }
    }
}
