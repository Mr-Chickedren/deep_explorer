// UNIX
use std::io::{stdout, Write};
use termion::{clear, cursor};
use termion::raw::IntoRawMode;

#[derive(Clone, PartialEq)]
enum Cell {
	Nop, // initial cell
	None,
	Human,
}
impl Cell {
	fn as_char(&self) -> char {
		match self {
			Cell::Nop => '.', // initial cell
			Cell::None => ' ',
			Cell::Human => '&',
		}
	}
}

struct Canvas {
	prev: Vec<Vec<Cell>>,
	current: Vec<Vec<Cell>>,
}
impl Canvas {
	fn init(width: usize, height: usize) -> Self {
		Self {
			prev: vec![vec![Cell::None;width];height],
			current: vec![vec![Cell::Nop;width];height],
		}
	}

	fn draw(&mut self) {
		// UNIX
		let mut stdout = stdout().into_raw_mode().unwrap();
		write!(stdout, "{}", clear::All).unwrap();

		for h in 0..self.current.len() {
			for w in 0..self.current[h].len() {
				if self.current[h][w] == self.prev[h][w] { continue }
				self.current[h][w] = self.prev[h][w].clone();
				write!(stdout, "{}{}", cursor::Goto(w as u16 + 1, h as u16 + 1), self.current[h][w].as_char()).unwrap();
			}
		}

		stdout.flush().unwrap();
	}
}

fn move_cursor(x: u16, y: u16) {
	// UNIX
	let mut stdout = stdout().into_raw_mode().unwrap();
	write!(stdout, "{}", cursor::Goto(x, y)).unwrap();
	stdout.flush().unwrap();
}

fn clear_terminal() {
	// UNIX
	let mut stdout = stdout().into_raw_mode().unwrap();
	write!(stdout, "{}", clear::All).unwrap();
	stdout.flush().unwrap();
}

fn main() {
	let mut canvas = Canvas::init(10,10);
	canvas.draw();
	//canvas.prev[0][0] = Cell::Human;
	canvas.draw();
}
