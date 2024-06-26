// TODO:
// - unicode support
// - faster rendering of text
//     - re-render only the changed lines
// - make document::piecetable::PieceTable::gen_string more efficient
// - keep cursur position when going from a larger to a smaller back to a larger line
// - ctr-delete
// - undo and redo

use std::{env, process::exit};

use editor::Editor;
use frontend::Frontend;

mod document;
mod editor;
mod frontend;

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

    let mut frontend = Frontend::new(editor);

    frontend.run().unwrap();
}
