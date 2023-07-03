use std::{
    env,
    io::Write,
    iter::Peekable,
    path::Path,
    process::{Child, Command, Stdio},
};
fn main() {
    loop {
        print!(">");
        std::io::stdout().flush().unwrap();
        let mut user_input = String::new();
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let mut commands: Peekable<std::str::Split<'_, &str>> =
            user_input.trim().split(" | ").peekable();
        handle_command(&mut commands)
    }
}
fn handle_command(commands: &mut Peekable<std::str::Split<'_, &str>>) {
    let mut prev_command: Option<Child> = None;
    while let Some(command) = commands.next() {
        let mut command_parts = command.trim().split_whitespace();
        let command = command_parts.next().unwrap();
        let args = command_parts;
        match command {
            "exit" => {
                std::process::exit(0);
            }
            "cd" => {
                let next_dir = args.peekable().peek().map_or("/", |s| *s);
                let path = Path::new(next_dir);
                match env::set_current_dir(path) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("Failed to change directory, {}", e);
                    }
                }
                prev_command = None;
            }
            command => {
                let stdin =
                prev_command.map_or(Stdio::inherit(), |output| Stdio::from(output.stdout.unwrap()));
                let stdout = if commands.peek().is_some() {
                    Stdio::piped()
                }
                else {
                    Stdio::inherit()
                };
                let child = Command::new(command).args(args).stdin(stdin).stdout(stdout).spawn();
                match child {
                    Ok(child) => {
                        prev_command = Some(child);
                    }
                    Err(e) => {
                        prev_command = None;
                        eprintln!("Failed to execute command, {}", e);
                    }
                }
            }
        }
    }
    if let Some(mut final_command) = prev_command {
        final_command.wait().unwrap();
    }
}


