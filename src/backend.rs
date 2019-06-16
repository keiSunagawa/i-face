use std::error::Error;
use std::io::Write;
use std::process::Child;

pub struct Proc {
    underlying: Child,
}

impl Proc {
    pub fn new(c: Child) -> Proc {
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
    pub fn wait(&mut self) -> Result<(), Box<Error>> {
        self.underlying.wait()?;
        Ok(())
    }
    pub fn kill(&mut self) -> Result<(), Box<Error>> {
        self.underlying.kill()?;
        Ok(())
    }
}
