/*
 *    RTFM is an open source program for printing a friendly reminder
 *    any time you have a question about a program on your computer system,
 *    or a piece of third party software you may be working with.
 *    It will even issue a friendly reminder about how to use RTFM itself.
 *
 *    Copyright (C) 2017-2018 Christopher Blanchard
 *
 *    This program is free software: you can redistribute it and/or modify
 *    it under the terms of the GNU General Public License as published by
 *    the Free Software Foundation, either version 3 of the License, or
 *    (at your option) any later version.
 *
 *    This program is distributed in the hope that it will be useful,
 *    but WITHOUT ANY WARRANTY; without even the implied warranty of
 *    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *    GNU General Public License for more details.
 *
 *    You should have received a copy of the GNU General Public License
 *    along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */
extern crate rand;

use std::process::Command;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io;
use std::io::Write;

#[cfg(target_os = "windows")]
use std::process::Stdio;

const INSULTS: [&str; 10] = [
    "RTFM!",
    "RTFM Noob!",
    "Seriously, RTFM!",
    "RTFM Already!",
    "Argh! Just RTFM! Do it NOW!",
    "RTFM Something', will ya!",
    "Nyaaaagh! The manual, read it now!",
    "Sudo read the manual.",
    "Sudo read the friendly manual.",
    "There is this wonderful thing you could try reading called the \"manual\"."
];

const CONFIG_FILE: &str = "rtfm.toml";


#[derive(Clone, PartialEq, Eq)]
enum Action {
    DefaultMessage,
    FetchManual(String),
    HelpPage,
    PrivacyPolicy,
}

fn run_help_page() {
    let i = rand::random::<usize>() % INSULTS.len();
    println!("{}", INSULTS[i]);
    println!("HINT: `rtfm rtfm` or `man rtfm`.");
    println!("Or do I need to do this for you?");
}

fn run_default_message() {
    let i = rand::random::<usize>() % INSULTS.len();
    println!("{}", INSULTS[i]);
}

#[cfg(target_os = "windows")]
fn make_command(program_name: &str) -> Option<Command> {
    // First, try calling `help`.
    let output = Command::new("help")
                         .arg(program_name)
                         .stdin(Stdio::null())
                         .stdout(Stdio::piped())
                         .output();

    if output.is_ok() {
        let unwrapped = output.unwrap();
        let success = !unwrapped.status.success();
        if success {
            let string = Vec::from("This command is not supported by the help utility.  Try");
            if !(unwrapped.stdout.starts_with(&string)) ||
                 !(unwrapped.stderr.starts_with(&string)
            ) {
                // If `help` succeeds, that's the command we'll use.
                let mut command = Command::new("help");
                command.arg(program_name);
                return Some(command);
            }
        }
    }
    // Otherwise, try calling the command with the '/?' flag.
    let output = Command::new(program_name)
                         .arg("/?")
                         .stdin(Stdio::null())
                         .stdout(Stdio::piped())
                         .output();

    if output.is_ok() && output.unwrap().status.success() {
        // If calling the process with the '/?' flag succeeds, that's the 
        // command we'll use.
        let mut command = Command::new(program_name);
        command.arg("/?");
        return Some(command);
    }

    // Otherwise, try calling the command with the "--help" flag.
    let output = Command::new(program_name)
                         .arg("--help")
                         .stdin(Stdio::null())
                         .stdout(Stdio::piped())
                         .output();

    if output.is_ok() && output.unwrap().status.success() {
        // If calling the process with the '--help' flag succeeds, that's the 
        // command we'll use.
        let mut command = Command::new(program_name);
        command.arg("--help");
        return Some(command);
    }

    // Failing that, we just give up.
    None
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn make_command(program_name: &str) -> Option<Command> {
    let mut command = Command::new("man");
    command.arg(&program_name);

    Some(command)
}

#[cfg(target_os = "windows")]
fn run_fetch_manual(program_name: &str) {
    println!("So you're having a problem with {}?", program_name);
    println!("Let me RTFM that for you.");

    let mut command = match make_command(program_name) {
        Some(val) => val,
        None => {
            println!(
                "Couldn't find a help page for \"{}\". \
                 You'll just have to RTFM somewhere else then.",
                 program_name
            );
            return ();
        }
    };
    if let Ok(mut child) = command.spawn() {
        match child.wait() {
            Ok(_) => {
                println!("There, was that so difficult now?");
            }
            Err(_) => {
                println!("Well fine, go RTFM somewhere else then!");
            }
        }
    } else {
        println!("Well fine, go RTFM somewhere else then!");
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn run_fetch_manual(program_name: &str) {
    println!("So you're having a problem with {}?", program_name);
    println!("Let me RTFM that for you.");

    let mut command = match make_command(program_name) {
        Some(val) => val,
        None => {
            println!(
                "Couldn't find a help page for {}. 
                 You'll just have to RTFM somewhere else.",
                 program_name
            );
            return ();
        }
    };
    if let Ok(mut child) = command.spawn() {
        match child.wait() {
            Ok(exit_code) => {
                match exit_code.code() {
                    Some(0) => {
                        println!("There, was that so difficult now?");
                    }
                    Some(_) | None => {
                        println!("Well fine, go RTFM somewhere else then!");
                    }
                }
            }
            Err(_) => {
                println!("Well fine, go RTFM somewhere else then!");
            }
        }
    } else {
        println!("Well fine, go RTFM somewhere else then!");
    }
}

fn run_privacy_policy() {
    println!("RTFM PRIVACY POLICY");
    println!("This ain't Google. RTFM doesn't collect your data.");
    println!("END PRIVACY POLICY")
}

fn parse_args(args: &[String]) -> Action {
    if args.len() < 2 {
        return Action::DefaultMessage;
    }

    if args.contains(&String::from("--privacy-policy")) {
        return Action::PrivacyPolicy;
    }

    if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        return Action::HelpPage;
    }

    Action::FetchManual(args[1].clone())
}

fn run_action(action: &Action) {
    match action {
        &Action::DefaultMessage => {
            run_default_message();
        }
        &Action::FetchManual(ref program_name) => {
            run_fetch_manual(&program_name);
        }
        &Action::HelpPage => {
            run_help_page();
        }
        &Action::PrivacyPolicy => {
            run_privacy_policy();
        }
    }
}

fn config_path(file_name: &str) -> String {
    let home = env::var("HOME").unwrap();
    format!("{}/.config/{}", &home, file_name)
}

fn make_default_config(file_name: &str, insults: &[&str]) -> io::Result<()> {
    let file_string = config_path(file_name);
    let file_path = Path::new(&file_string);
    let mut config_file = File::create(file_path)?;
    
    config_file.write(b"# Place your insults here\n")?;
    config_file.write(b"[insults]\n")?;
    for string in insults.iter() {
        config_file.write(string.as_ref())?;
        config_file.write(b"\n")?;
    }

    config_file.write(b"\n")?;
    
    Ok(())
}

fn config_exists() -> bool {
    let home = env::var("HOME").unwrap();
    let config_path = format!("{}/.config/{}", &home, CONFIG_FILE);
    let config_file = Path::new(&config_path);
    if config_file.exists() {
        return true;
    } else {
        return false;
    }
}

fn main() {
    if !config_exists() {
        println!("Config file does not exist! How am I supposed to insult you?");
        println!("I know! I will generate a default.");
        make_default_config(CONFIG_FILE, &INSULTS).unwrap();
        let file_path = config_path(CONFIG_FILE);
        println!("Default configuration generated at {}", file_path);
    }

    let args: Vec<String> = env::args().collect();
    let action = parse_args(&args);
    run_action(&action);
}
