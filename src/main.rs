/*
 *    RTFM is an open source program for printing a friendly reminder
 *    any time you have a question about a program on your computer system,
 *    or a piece of third party software you may be working with.
 *    It will even issue a friendly reminder about how to use RTFM itself.
 *
 *    Copyright (C) 2017  Christopher Blanchard
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

use std::process;
use std::env;


const STRINGS: [&str; 9] = [
    "RTFM!",
    "RTFM Noob!",
    "Seriously, RTFM!",
    "RTFM Already!",
    "Argh! Just RTFM! Do it NOW!",
    "Nyaaaagh! The manual, read it now!",
    "Sudo read the manual.",
    "Sudo read the friendly manual.",
    "There is this wonderful thing you could try reading called the \"manual\"."
];

#[derive(Clone, PartialEq, Eq)]
enum TerminalAction {
    DefaultMessage,
    FetchManual(String),
    HelpPage,
}

fn run_help_page() {
    let i = rand::random::<usize>() % STRINGS.len();
    println!("{}", STRINGS[i]);
    println!("HINT: try running `rtfm rtfm` or `man rtfm`.");
    println!("Or do I need to do this for you?");
}

fn run_default_message() {
    let i = rand::random::<usize>() % STRINGS.len();
    println!("{}", STRINGS[i]);
}

fn run_fetch_manual(program_name: &str) {
    println!("So you're having a problem with {}?", program_name);
    println!("Let me RTFM that for you.");

    let mut command = process::Command::new("man");
    command.arg(&program_name);
    
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

fn parse_args(args: &[String]) -> TerminalAction {
    if args.len() < 2 {
        return TerminalAction::DefaultMessage;
    }
    if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        return TerminalAction::HelpPage;
    }

    TerminalAction::FetchManual(args[1].clone())
}

fn run_action(action: &TerminalAction) {
    match action {
        &TerminalAction::DefaultMessage => {
            run_default_message();
        }
        &TerminalAction::FetchManual(ref program_name) => {
            run_fetch_manual(&program_name)
        }
        &TerminalAction::HelpPage => {
            run_help_page()
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let action = parse_args(&args);
    run_action(&action);
}
