use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::cursor::DetectCursorPos;
use rand::{seq::SliceRandom, Rng};

fn main() {
	let stdin = stdin();
	let mut stdout = stdout().into_raw_mode().unwrap();

	for key in stdin.keys() {
		match key.unwrap() {
			Key::Char(ch) => {
				let termsize::Size {rows, cols} = termsize::get().unwrap();
				let items = vec!["\x1b[31m", "\x1b[32m", "\x1b[33m", "\x1b[34m", "\x1b[35m", "\x1b[36m", "\x1b[37m", "\x1b[0m"];
				match items.choose(&mut rand::thread_rng()){
					Some(item) => print!("{}", item),
					None => print!("\x1b[0m")
				};
				print!("\n\x1b[2K{} \x1b[0m", ch);
				match stdout.cursor_pos() {
					Ok(coords) => if coords.0 >= cols {
						print!("\x1b[0G");
					},
					Err(err) => panic!("{}", err)
				}
			},
			Key::Backspace => {
				println!("\x1b[0G");
				break;
			},
			Key::Ctrl(_) | Key::Alt(_) => print!(""),
			_ => print!("?")
		}

		stdout.flush().unwrap();
	}
}
