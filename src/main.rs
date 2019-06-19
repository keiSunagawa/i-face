use i_face::backend::{Backend, Proc};
use i_face::frontend::Front;
use std::env;
use std::error::Error;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn main() {
    run().expect("Error!!!");
}

fn run() -> Result<(), Box<Error>> {
    let mut com_and_args = env::args().collect::<Vec<String>>().split_off(1);
    let com = com_and_args
        .pop()
        .expect("needs args length to one or more");
    println!("++command is {}", &com);
    let child = Command::new(com)
        .args(com_and_args.as_slice())
        .stdin(Stdio::piped())
        .spawn()?;

    let mut p = Proc::new(child);
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
