mod tokenize;
use tokenize::tokenize;
use std::io::{self, Write};
use std::process::Command;
use std::fs::File;

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
                let mut args = args.to_vec();
                let mut out_file: Option<File> = None;

                if let Some(pos) = args.iter().position(|x| x == ">") {
                    if pos + 1 < args.len() {
                        let filename = args[pos + 1].clone();
                        match File::create(&filename) {
                            Ok(file) => {
                                out_file = Some(file);
                                args.drain(pos..=pos + 1);
                            }
                            Err(e) => {
                                eprintln!("myshell: cannot create file {}: {}", filename, e);
                                continue;
                            }
                        }
                    } else {
                        eprintln!("myshell: syntax error near unexpected token `newline'");
                        continue;
                    }
                }

                // ส่วนที่หายไป: รันโปรแกรมจริง
                let mut command = Command::new(cmd);
                command.args(&args);
                if let Some(file) = out_file {
                    command.stdout(file);   // หัวใจของ redirect
                }
                if let Err(e) = command.status() {
                    eprintln!("myshell: {cmd}: {e}");
                }
            }
        } 
    } 
}