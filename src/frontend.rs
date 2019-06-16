extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Front {
    underlying: Editor<()>,
}

impl Front {
    pub fn new() -> Front {
        let mut inst = Front {
            underlying: Editor::<()>::new(),
        };
        if inst.underlying.load_history("history.txt").is_err() {
            println!("No previous history.");
        }
        inst
    }
    pub fn read(&mut self) -> String {
        let res = self.underlying.readline("");
        res.expect("ooooo!!!")
    }
    pub fn read_loop(&mut self, k: &mut FnMut(String) -> ()) {
        loop {
            match self.underlying.readline("") {
                Ok(line) => {
                    self.underlying.add_history_entry(line.as_ref());
                    k(line);
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
        self.underlying.save_history("history.txt").unwrap();
    }
}
