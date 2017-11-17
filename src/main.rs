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

use std::env;


const STRINGS: [&str; 8] = [
    "RTFM!",
    "RTFM Noob!",
    "Seriously, RTFM!",
    "RTFM Already!",
    "Argh! Just RTFM! Do it NOW!",
    "Nyaaaagh! The manual, read it now!",
    "Sudo read the manual.",
    "There is this wonderful thing you could try reading called the \"manual\"."
];

#[derive(Clone, PartialEq, Eq)]
enum TerminalAction {
    DefaultMessage,
    FetchManual(String),
    HelpPage,
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

fn run_help_page() {
    println!("There is this wonderful thing you could \
              try reading called the \"manual\".");
    println!("Try running:");
    println!("`rtfm rtfm` or `man rtfm`.");
}

fn run_default_message() {
    let i = rand::random::<usize>() % STRINGS.len();
    println!("{}", STRINGS[i]);
}

fn run_fetch_manual(program_name: &str) {
    unimplemented!();
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
