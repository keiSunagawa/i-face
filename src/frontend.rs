extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Front {
    underlying: Editor<()>
}

impl Front {
    pub fn new() -> Front {
        let mut inst = Front {
            underlying: Editor::<()>::new()
        };
        if inst.underlying.load_history("history.txt").is_err() {
            println!("No previous history.");
        }
        inst
    }

    pub fn read(& mut self) -> String {
        let res = self.underlying.readline(">> ");
        res.expect("ooooo!!!")
    }
}
