use std::io::{self, Write};

use crate::query;

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
                let _ = query::parse(&buf).unwrap();
            }
        }
    }

    Ok(())
}
