use std::io::{self, Write};
use std::{env, path::{Path, PathBuf}, process::{self, Command, ExitCode}};
use gethostname::gethostname;

fn find_bin(name: &str) -> Option<PathBuf> {
    if let Ok(paths) = env::var("PATH") {
        for path in env::split_paths(&paths) {
            let bin_path = path.join(name);
            if bin_path.is_file() {
                return Some(bin_path);
            }
        }
    }

    None
}

fn main() -> ExitCode{
    let path_env = std::env::var("PATH").unwrap();
    let user = std::env::var("USER").unwrap();
    let mut user_home_str: String = "/home/".to_owned();
    user_home_str.push_str(&user);

    loop {
        // set up PS1
        let hostname = gethostname().into_string().unwrap();
        let current_working_dir = env::current_dir().unwrap();

        print!("{user}@{hostname}:{}$ ", current_working_dir.to_str().unwrap().replace(&user_home_str, "~"));
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let inputs: Vec<&str> = input
            .lines()
            .next()
            .unwrap_or("")
            .split_whitespace()
            .collect();

        match inputs[..] {
            ["exit", code] => {
                if code == "0" {
                    process::exit(0);
                }
            }

            ["echo", ..] => println!("{}", inputs[1..].join(" ")),

            ["pwd"] => {
                let working_dir = env::current_dir().unwrap();
                println!("{}", working_dir.display());
            }

            ["cd"] => {
                let home = env::var("HOME").unwrap();
                if std::env::set_current_dir(&home).is_err() {
                    println!("cd: The directory '{home}' does not exist");
                }
            }

            ["cd", path] => {
                if path == "~" {
                    let home = env::var("HOME").unwrap();
                    if std::env::set_current_dir(&home).is_err() {
                        println!("cd: The directory '{home}' does not exist");
                    }
                } else {
                    if std::env::set_current_dir(Path::new(path)).is_err() {
                        println!("cd: The directory '{path}' does not exist");
                    }
                }
            }

            ["type", command] => {
                match command {
                    "exit" | "echo" | "type" | "pwd" => println!("{command} is a shell builtin"),

                    _ => {
                        let bin_dirs = &mut path_env.split(':');
                        
                        if let Some(path) = bin_dirs
                            .find(|path| std::fs::metadata(format!("{}/{}", path, command)).is_ok())
                        {
                            println!("{command} is {path}/{command}");
                        } else {
                            println!("{command}: not found");
                        }
                    }
                }
            }

            _ if inputs.len() > 0 => {
                if let Some(bin) = find_bin(inputs[0]) {
                    let args = &inputs[1..];
                    args.join(" ");

                    Command::new(bin).args(args)
                        .status()
                        .expect(
                            "Failed to execute binary"
                        );
                } else {
                    println!("{}: command not found", input.trim());
                }
            }

            _ => {}
        }
    }
}
