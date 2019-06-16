use i_face::backend::Proc;
use i_face::frontend::Front;
use std::env;
use std::error::Error;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn main() {
    run();
    println!("Hello, world!");
}

fn run() -> Result<(), Box<Error>> {
    let mut args: Vec<String> = env::args().collect();
    println!("++args is {:?}", &args);
    args.reserve(0);
    let com = args.pop().expect("needs args length to one or more");
    println!("++command is {}", &com);
    let mut child = Command::new(com).stdin(Stdio::piped()).spawn()?;

    let mut p = Proc::new(&mut child);
    let mut f = Front::new();
    let mut c = |s: String| -> () {
        let ns = format!("{}{}", s, "\n");
        p.send(ns.as_bytes());
    };
    f.read_loop(&mut c);
    thread::sleep(Duration::from_secs(1));
    p.kill()?;
    Ok(())
}
