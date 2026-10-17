# myshell

A small Unix shell written in Rust, built while learning the language.

```
myshell> cd ~/projects
myshell> ls -la > files.txt
myshell> echo "hello world"
hello world
myshell> exit
```

## Features

- Runs external programs with arguments (`ls -la`, `git status`, ...)
- Built-in `cd` (no argument goes to `$HOME`) and `exit [code]`
- `~` and `~/path` expand to your home directory
- Double quotes keep spaces inside one argument: `echo "hello world"`
- Output redirection with `>`: `ls > files.txt`
- Line editing and persistent history (saved to `~/.myshell_history`)
- **Up/Down arrows search history by prefix**: type `git` then press Up to cycle through earlier `git ...` commands
- `Ctrl+C` clears the current line, `Ctrl+D` exits
- Friendly errors, e.g. `myshell: foo: command not found`

## Build and run

You need a Rust toolchain (edition 2024).

```sh
git clone https://github.com/polp45348-glitch/myshell
cd myshell
cargo run
```

## Known limitations

This is a learning project, and I know about these gaps:

- **No pipes yet** (`ls | grep rs` does not work)
- **No input redirection** (`<`) or append (`>>`)
- **Pressing `Ctrl+C` while a program is running also exits myshell**, because the shell doesn't handle `SIGINT` yet
- **The tokenizer forgets quoting**, so a quoted `">"` is still treated as redirection, `ls>out.txt` without spaces isn't redirected, and an empty argument `""` is dropped
- Single quotes and backslash escapes are not supported
- No environment variables (`$HOME`, `$?`) and no job control (`&`, `fg`, `bg`)

## Roadmap

- [ ] Handle `Ctrl+C` so only the running program is interrupted
- [ ] Tokenizer that returns typed tokens (`Word`, `Pipe`, `RedirOut`, ...) instead of plain strings
- [ ] Pipes (`|`)
- [ ] `<` and `>>` redirection
- [ ] `$?` and environment variable expansion

## Project layout

```
src/main.rs       read-eval loop, built-ins, redirection, running commands
src/tokenize.rs   splits a line into arguments, handles double quotes
```
