use std::error::Error;
use std::io::Write;
use std::process::Child;

pub trait Backend {
    fn send(&mut self, msg: &[u8]) -> Result<(), Box<Error>>;
    fn wait(&mut self) -> Result<(), Box<Error>>;
    fn kill(&mut self) -> Result<(), Box<Error>>;
}

pub struct Proc {
    underlying: Child,
}

impl Proc {
    pub fn new(c: Child) -> Proc {
        Proc { underlying: c }
    }
}
impl Backend for Proc {
    fn send(&mut self, msg: &[u8]) -> Result<(), Box<Error>> {
        self.underlying
            .stdin
            .as_mut()
            .ok_or("Child process stdin has not been captured!")?
            .write_all(msg)?;
        Ok(())
    }
    fn wait(&mut self) -> Result<(), Box<Error>> {
        self.underlying.wait()?;
        Ok(())
    }
    fn kill(&mut self) -> Result<(), Box<Error>> {
        self.underlying.kill()?;
        Ok(())
    }
}

mod reader;
// pub struct SubProc {
//     underlying: Popen,
// }

// impl SubProc {
//     pub fn new(c: Popen) -> SubProc {
//         SubProc { underlying: c }
//     }
// }
// impl Backend for SubProc {
//     fn send(&mut self, msg: &[u8]) -> Result<(), Box<Error>> {
//         let s = String::from_utf8(msg.to_vec()).unwrap();
//         let msg_m = Some(s.as_str());
//         let (out, _err) = self.underlying.communicate(msg_m)?;
//         println!("{:?}", &out);
//         Ok(())
//     }
//     fn wait(&mut self) -> Result<(), Box<Error>> {
//         self.underlying.wait()?;
//         Ok(())
//     }
//     fn kill(&mut self) -> Result<(), Box<Error>> {
//         self.underlying.kill()?;
//         Ok(())
//     }
// }
