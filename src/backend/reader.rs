#[derive(PartialOrd, PartialEq, Debug)]
pub enum ReadResult {
    Continue,
    Line(String),
    Prompt,
}
fn read_byte(next: u8, buf: &mut Vec<u8>, prompt: &[u8]) -> ReadResult {
    // 値をproptの末尾と比較
    let last = prompt.last().expect("must prompt non empty!");
    if last == &next && match_prompt(buf, prompt) {
        // 一致した場合はPromptを返す
        ReadResult::Prompt
    } else if next == 0x0A {
        // 改行の場合はLineにくるんで返す
        let str = String::from_utf8(buf.clone()).expect("TODO safe string dcode");
        buf.clear();
        ReadResult::Line(str)
    } else {
        // そうでない場合はbufに追加してContを返す
        buf.push(next);
        ReadResult::Continue
    }
}
// 値が0の場合はEOFをreturn

// 一致した場合はpromptの長さ分bufから取り出して比較
fn match_prompt(buf: &Vec<u8>, prompt: &[u8]) -> bool {
    let n = prompt.len() - 1;
    if buf.len() == n {
        let p: Vec<&u8> = prompt.iter().take(prompt.len() - 1).collect();
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
    let buf = "scala";
    let prompt = "scala>";
    let next = ">";
    let res = read_byte(
        next.as_bytes().iter().next().unwrap().clone(),
        &mut buf.as_bytes().to_vec(),
        &prompt.as_bytes(),
    );
    assert_eq!(res, ReadResult::Prompt);
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
