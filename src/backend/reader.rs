use std::io::Read;

#[derive(PartialOrd, PartialEq, Debug)]
pub enum ReadResult {
    Continue(Vec<u8>),
    Line(String),
    Prompt { remaining: String }, // res remaing
    EOF,
}

pub struct StreamReader {
    stream: Box<Read + Send>,
}

impl StreamReader {
    pub fn new(sm: Box<Read + Send>) -> Self {
        StreamReader { stream: sm }
    }
    pub fn read(&mut self, mut buf: Vec<u8>, prompt: &[u8]) -> ReadResult {
        let mut bt = [0u8];
        match self.stream.read(&mut bt) {
            Ok(0) => ReadResult::EOF,
            Ok(1) => read_byte(bt[0], buf, prompt),
            a => {
                println!("invalid res {:?}", &a);
                panic!("read stream error!")
            }
        }
    }
}
fn read_byte(next: u8, mut buf: Vec<u8>, prompt: &[u8]) -> ReadResult {
    // 値をproptの末尾と比較
    let last = prompt.last().expect("must prompt non empty!");
    if last == &next && match_prompt(&buf, prompt) {
        // 一致した場合はPromptを返す
        let rem = if buf.len() > prompt.len() - 1 {
            let n = buf.len() - (prompt.len() - 1);
            let b = buf.iter().take(n).cloned().collect::<Vec<u8>>();
            let res = String::from_utf8(b).unwrap();
            res
        } else {
            "".to_string()
        };
        ReadResult::Prompt { remaining: rem }
    } else if next == 0x0A {
        // 改行の場合はLineにくるんで返す
        let str = String::from_utf8(buf.clone()).expect("TODO safe string dcode");
        ReadResult::Line(str)
    } else {
        // そうでない場合はbufに追加してContを返す
        buf.push(next);
        ReadResult::Continue(buf)
    }
}
// 値が0の場合はEOFをreturn

// 一致した場合はpromptの長さ分bufから取り出して比較
fn match_prompt(buf: &Vec<u8>, prompt: &[u8]) -> bool {
    let n = prompt.len() - 1;

    if buf.len() >= n {
        let p: Vec<&u8> = prompt.iter().take(n).collect();
        let b: Vec<&u8> = buf.iter().skip(buf.len() - n).collect();
        p == b
    } else {
        false
    }
}

#[test]
fn test_match_prompt() {
    let buf = "scala";
    let prompt = "scala>";
    let bufp = buf.as_bytes().to_vec();
    let promptp = prompt.as_bytes();
    let res = match_prompt(&bufp, &promptp);
    assert!(res, "unmatch prompt. {:?} !== {:?}", &buf, &prompt);
}
#[test]
fn test_read_byte_match_prompt() {
    let buf = "go scala";
    let prompt = "scala>";
    let next = ">";
    let res = read_byte(
        next.as_bytes().iter().next().unwrap().clone(),
        &mut buf.as_bytes().to_vec(),
        &prompt.as_bytes(),
    );
    assert_eq!(
        res,
        ReadResult::Prompt {
            remaining: "go ".to_string()
        }
    );
}
#[test]
fn test_read_byte_match_new_line() {
    let buf = "this message print for std output";
    let prompt = "scala>";
    let next = "\n";
    let res = read_byte(
        next.as_bytes().iter().next().unwrap().clone(),
        &mut buf.as_bytes().to_vec(),
        &prompt.as_bytes(),
    );
    assert_eq!(res, ReadResult::Line(buf.to_string()));
}
#[test]
fn test_read_byte_unmatch_to_continue() {
    let buf = "this message print for std output";
    let prompt = "scala>";
    let next = "z";
    let mut actual = buf.as_bytes().to_vec().clone();
    let res = read_byte(
        next.as_bytes().iter().next().unwrap().clone(),
        &mut actual,
        &prompt.as_bytes(),
    );
    assert_eq!(res, ReadResult::Continue);
    let actual_string = String::from_utf8(actual).expect("not utf8 value.");
    assert_eq!(actual_string, format!("{}{}", buf, next));
}
