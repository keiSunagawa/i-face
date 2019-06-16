use std::error::Error;
use std::io::Read;
use std::io::Write;
use std::process::Child;

pub struct Proc<'a> {
    underlying: &'a mut Child,
}

impl<'a> Proc<'a> {
    pub fn new(c: &mut Child) -> Proc {
        Proc { underlying: c }
    }
    pub fn send(&mut self, msg: &[u8]) -> Result<(), Box<Error>> {
        self.underlying
            .stdin
            .as_mut()
            .ok_or("Child process stdin has not been captured!")?
            .write_all(msg)?;
        Ok(())
    }
    // pub fn read(&mut self) -> String {
    //     let mut res = String::new();
    //     self.underlying
    //         .stdout
    //         .as_mut()
    //         .expect("Child process stdin has not been captured!")
    //         .read_to_string(&mut res);
    //     res
    // }
    pub fn wait(&mut self) -> Result<(), Box<Error>> {
        self.underlying.wait()?;
        Ok(())
    }
    pub fn kill(&mut self) -> Result<(), Box<Error>> {
        self.underlying.kill()?;
        Ok(())
    }
}
