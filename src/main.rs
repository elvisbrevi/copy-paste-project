extern crate clipboard;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut last_contents = String::new();

    loop {
        // Obtén el contenido actual del portapapeles.
        match ctx.get_contents() {
            Ok(contents) => {
                // Comprueba si el contenido ha cambiado desde la última vez.
                if contents != last_contents {
                    println!("Nuevo contenido detectado en el portapapeles! {}", contents);
                    last_contents = contents;
                }
            }
            Err(e) => println!("Error al acceder al portapapeles: {:?}", e),
        }

        // Espera un poco antes de comprobar de nuevo.
        sleep(Duration::from_secs(2));
    }
}
