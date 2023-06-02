use std::io::{BufRead, BufReader, Read, Write};

use crate::lexer::Lexer;

const PROMPT: &str = ">>";

pub fn start<R: Read, W: Write>(r#in: R, out: &mut W) {
    let mut reader = BufReader::new(r#in);

    let mut line = String::new();
    loop {
        write!(out, "{} ", &PROMPT).expect("Failed to write to out");
        out.flush().expect("Failed to flush out");

        reader.read_line(&mut line).expect("Failed to read line");

        let lexer = Lexer::new(&line);
        for token in lexer {
            writeln!(out, "{:?}", token).expect("Failed to write to out");
        }

        out.flush().expect("Failed to flush out");
        line.clear();
    }
}
