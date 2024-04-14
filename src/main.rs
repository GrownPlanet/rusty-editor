/*
* . src
* |- editor               ; no big main file :)
*   |- document           ; contains the document
*       |- piece table    ; contains the content of the document
*   |- frontent           ; bevy and rendering
*       |- ???            ; idk yet
* */

use std::{env, process::exit};

use editor::Editor;

mod document;
mod editor;
mod frontend;
mod piecetable;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: rusty_editor <filename>");
        exit(-1);
    }

    let editor = match Editor::new(&args[1]) {
        Ok(e) => e,
        Err(m) => {
            println!("Error creating editor: {}", m);
            exit(-1);
        }
    };
}
