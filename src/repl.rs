use std::io::{Stdin, Stdout, Write};

use crate::lexericals::{lexer::Lexer, tokenizer::Tokenkind};

pub fn start(stdin: Stdin, mut stdout: Stdout) {
    loop {
        write!(stdout, ">>> ").expect("Failed to write >> to stdout");
        stdout.flush().expect("Failed to flush stdout");

        let mut input = String::new();

        if let Err(e) = stdin.read_line(&mut input) {
            writeln!(stdout, "Error reading input: {}", e)
                .expect("Failed to write error to stdout");
            stdout.flush().expect("Failed to flush stdout");
            return;
        }
        let mut lexer = Lexer::new(&input);
        loop {
            let token = lexer.next_token();
            if token.kind == Tokenkind::Eof {
                break;
            }
            writeln!(stdout, "Token: {:?}", token).expect("Failed to write token to stdout");
            stdout.flush().expect("Failed to flush stdout");
        }
        writeln!(stdout, "End of input").expect("Failed to write end of input to stdout");
        stdout.flush().expect("Failed to flush stdout");
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_start() {
//         let input = "let x = 5;\nlet y = 10;\n";
//         let mut stdin = std::io::Cursor::new(input);
//         let mut stdout = std::io::Cursor::new(Vec::new());

//         start(stdin, stdout);

//         let output = String::from_utf8(stdout.into_inner()).expect("Failed to convert output to string");
//         assert!(output.contains("Token:"));
//         assert!(output.contains("End of input"));
//     }
// }
