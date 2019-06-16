use i_face::backend::new;
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
    let mut child = Command::new(com)
        .stdin(Stdio::piped())
        //        .stdout(Stdio::piped())
        //       .stderr(Stdio::piped())
        .spawn()?;

    let mut p = new(&mut child);
    let mut f = Front::new();
    let s = f.read();
    let ns = format!("{}{}", s, "\n");
    p.send(ns.as_bytes());
    // let res = p.read();
    // println!("{}", &res);
    thread::sleep(Duration::from_secs(1));
    p.wait()?;
    Ok(())
}
