use std::io::{self, BufRead, Write};

pub fn is_shuffle<T: PartialEq>(v1: &[T], v2: &[T]) -> bool {
    !v1.starts_with(v2)
}
pub fn output_name<W: Write>(w: &mut W, name: &str) -> io::Result<()> {
    writeln!(w, "name: {}", name)
}

pub fn prompt<R, W>(mut reader: R, mut writer: W, question: impl Into<String>) -> String
where
    R: BufRead,
    W: Write,
{
    writeln!(&mut writer, "{}", question.into()).expect("Unable to write");
    let mut s = String::new();
    reader.read_line(&mut s).expect("Unable to read");
    s
}

#[test]
fn test_with_in_memory() {
    let input = b"I'm George";
    let mut output = Vec::new();

    let answer = prompt(&input[..], &mut output, "Who goes there?");

    let output = String::from_utf8(output).expect("Not UTF-8");

    assert_eq!(output, "Who goes there?\n");
    assert_eq!(answer, "I'm George");
}
