use std::io::{self, Write};

use crate::query::{self, Token};

fn print_version() {
    println!(
        "{}",
        "\
mini-sql v0.1.0
author <alahmadrosid@gmail.com>
\\h - Show list command
"
    )
}

fn print_command() {
    println!(
        "{}",
        "\
\\h - Show list command
\\q - Close prompt
    "
    )
}

pub fn run() -> io::Result<()> {
    let stdin = io::stdin();
    print_version();

    loop {
        print!("mini-sql> ");
        let _ = io::stdout().flush();

        let mut buf = String::new();
        stdin.read_line(&mut buf)?;
        match &buf.trim()[..] {
            "\\q" => break,
            "\\h" => print_command(),
            _ => {
                let tokens = query::parse(&buf).unwrap();
                match tokens.get(0).unwrap() {
                    Token::SELECT => {
                        println!("Execute select query");
                    }
                    Token::CREATE => {
                        println!("Execute create query");
                    }
                    Token::INSERT => {
                        println!("Execute insert query");
                    }
                    Token::DELETE => {
                        println!("Execute delete query");
                    }
                    _ => {
                        println!("Unrecognized query {}", buf);
                    }
                }
            }
        }
    }

    Ok(())
}
