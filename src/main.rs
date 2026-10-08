mod tokenize;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use rustyline::{Cmd, KeyCode, KeyEvent, Modifiers};
use std::fs::File;
use std::process::Command;
use tokenize::tokenize;
fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    rl.bind_sequence(
        KeyEvent(KeyCode::Up, Modifiers::NONE),
        Cmd::HistorySearchBackward,
    );
    rl.bind_sequence(
        KeyEvent(KeyCode::Down, Modifiers::NONE),
        Cmd::HistorySearchForward,
    );
    let history_path = std::env::var("HOME")
        .map(|home| format!("{home}/.myshell_history"))
        .unwrap_or_default();
        let _ = rl.load_history(&history_path);
    loop {
        let input = match rl.readline("myshell> ") {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                let _ = rl.save_history(&history_path);
                line
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(e) => {
                eprintln!("myshell : {e}");
                break;
            }
        };

        let tokens = tokenize(&input)
        .iter()
        .map(|s| expand_tilde(s))
        .collect::<Vec<String>>();

        let Some((cmd, args)) = tokens.split_first() else {
            continue;
        };

        match cmd.as_str() {
            "exit" => {
                break;
            }
            "cd" => {
                let target = match args.first() {
                    Some(dir) => expand_tilde(dir),
                    None => match std::env::var("HOME") {
                        Ok(home) => home,
                        Err(_) => {
                            eprintln!("myshell: cd: HOME not set");
                            continue;
                        }
                    },
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

                let mut command = Command::new(cmd);
                command.args(&args);
                if let Some(file) = out_file {
                    command.stdout(file);
                }
                if let Err(e) = command.status() {
                    eprintln!("myshell: {cmd}: {e}");
                }
            }
        }
    }
    let _ = rl.save_history(&history_path);
} fn expand_tilde(s: &str) -> String {
    let Ok(home) = std::env::var("HOME") else {
        return s.to_string();
    };
    if s == "~" {
        home
    } else if let Some(rest) = s.strip_prefix("~/") {
        format!("{home}/{rest}")
    } else {
        s.to_string()
    }
}
