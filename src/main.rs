use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        print!("myshell> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }

        let mut parts = input.split_whitespace();
        let Some(cmd) = parts.next() else { continue };
        let args: Vec<&str> = parts.collect();

        match cmd {
            "exit" => {
                break;
            }
            "cd" => {
                if let Some(dir) = args.get(0) {
                    if let Err(e) = std::env::set_current_dir(dir) {
                        eprintln!("myshell: cd: {e}");
                    }
                } else {
                    eprintln!("myshell: cd: missing argument");
                }
            }
            _ => {
                if let Err(e) = Command::new(cmd).args(&args).status() {
                    eprintln!("myshell: {cmd}: {e}");
                }
            }
        }
    }
}