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


const STRINGS: [&str; 5] = [
    "RTFM!",
    "RTFM Noob!",
    "Seriously, RTFM!",
    "RTFM Already!",
    "Argh! Just RTFM! Do it NOW!",
];

fn main() {
    let i = rand::random::<usize>() % STRINGS.len();
    println!("{}", STRINGS[i]);
}
