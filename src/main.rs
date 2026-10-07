mod tokenize;
use tokenize::tokenize;

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

        let tokens = tokenize(&input);
        let Some((cmd, args)) = tokens.split_first() else { continue };

        match cmd.as_str() {
            "exit" => {
                break;
            }
            "cd" => {
                let target = match args.first() {
                    Some(dir) => dir.to_string(),
                    None => match std::env::var("HOME") {
                        Ok(home) => home,
                        Err(_) => {
                            eprintln!("myshell: cd: HOME not set");
                            continue;
                        }
                    }
                };
                if let Err(e) = std::env::set_current_dir(&target) {
                    eprintln!("myshell: cd: {e}");
                }
            }
            _ => {
                let mut child = match Command::new(cmd).args(args).spawn() {
                    Ok(child) => child,
                    Err(e) => {
                        eprintln!("myshell: {e}");
                        continue;
                    }
                };
                if let Err(e) = child.wait() {
                    eprintln!("myshell: {e}");
                }
            }
            
        }
    }
}