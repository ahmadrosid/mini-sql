use std::io::{self, Write};

use crate::{query::{self, Token}, document::Table};

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

    let mut table = Table::new();
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
                if tokens.is_empty() {
                    break;
                }
                
                match tokens.get(0).unwrap() {
                    Token::INSERT => {
                        let values = vec!["1", "alahmadrosid@gmail.com"];
                        table.insert(values);
                    }
                    Token::SELECT => {
                        table.select();
                    }
                    Token::CREATE => {
                        println!("Execute create query");
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
