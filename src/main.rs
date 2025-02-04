// UNIX
use std::io::{stdout, Write};
use termion::{clear, cursor};
use termion::raw::{IntoRawMode, RawTerminal};

fn move_cursor(stdout: &mut RawTerminal<std::io::Stdout>, x: u16, y: u16) {
	// UNIX
	write!(stdout, "{}", cursor::Goto(x, y)).unwrap();
}

fn clear_terminal(stdout: &mut RawTerminal<std::io::Stdout>) {
	// UNIX
	write!(stdout, "{}", clear::All).unwrap();
}

fn put_char(stdout: &mut RawTerminal<std::io::Stdout>, c: char) {
	write!(stdout, "{}", c).unwrap();
}

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
			Cell::None => '.',
			Cell::Human => '&',
		}
	}
}

struct Canvas {
	prev: Vec<Vec<Cell>>,
	current: Vec<Vec<Cell>>,
	stdout: RawTerminal<std::io::Stdout>,
	end_of_canvas: u16,
}
impl Canvas {
	fn init(width: usize, height: usize) -> Self {
		Self {
			prev: vec![vec![Cell::None;width];height],
			current: vec![vec![Cell::Nop;width];height],
			stdout: stdout().into_raw_mode().unwrap(),
			end_of_canvas: height as u16,
		}
	}

	fn clear(&mut self) {
		clear_terminal(&mut self.stdout);
	}

	fn draw(&mut self) {
		for h in 0..self.current.len() {
			for w in 0..self.current[h].len() {
				if self.current[h][w] == self.prev[h][w] { continue }
				self.current[h][w] = self.prev[h][w].clone();

				move_cursor(&mut self.stdout, w as u16 + 1, h as u16 + 1);
				put_char(&mut self.stdout, self.current[h][w].as_char());
			}
		}

		move_cursor(&mut self.stdout, 1, self.end_of_canvas + 1);
		self.stdout.flush().unwrap();
	}
}

fn main() {
	let mut canvas = Canvas::init(10,10);
	canvas.clear();
	canvas.draw();
	//canvas.prev[0][0] = Cell::Human;
	canvas.draw();
}
