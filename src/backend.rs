use std::error::Error;
use std::io::Write;
use std::process::Child;

mod reader;
use self::reader::{ReadResult, StreamReader};

pub trait Backend {
    fn send(&mut self, msg: &[u8]) -> Result<(), Box<Error>>;
    fn wait(&mut self) -> Result<(), Box<Error>>;
    fn kill(&mut self) -> Result<(), Box<Error>>;
}

pub struct Proc {
    underlying: Child,
    stream: StreamReader,
}

impl Proc {
    pub fn new(mut c: Child) -> Proc {
        let sm = c.stdout.take().expect("unread stdout on child process.");
        let reader = StreamReader::new(Box::new(sm));
        let mut p = Proc {
            underlying: c,
            stream: reader,
        };
        p.render();
        p
    }
    fn render(&mut self) {
        let mut buf = Vec::new();
        let p = "scala> ".as_bytes(); // TODO struct param
        loop {
            // FIXME: clone ><
            match self.stream.read(buf.clone(), p) {
                ReadResult::EOF => {
                    buf.clear();
                    break;
                }
                ReadResult::Continue(nbuf) => {
                    buf = nbuf;
                }
                ReadResult::Line(l) => {
                    buf.clear();
                    println!("{}", &l);
                }
                ReadResult::Prompt { remaining: r } => {
                    buf.clear();
                    print!("{}", &r);
                    break;
                }
            }
        }
    }
}
impl Backend for Proc {
    fn send(&mut self, msg: &[u8]) -> Result<(), Box<Error>> {
        self.underlying
            .stdin
            .as_mut()
            .ok_or("Child process stdin has not been captured!")?
            .write_all(msg)?;

        self.render();
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
