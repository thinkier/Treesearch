#[cfg(test)]
mod tests;
#[cfg(test)]
mod benches;

use rand;
use std::fmt::{self, Debug};
#[cfg(feature = "eyecandy")]
use std::fmt::Display;
#[cfg(feature = "eyecandy")]
use ansi_term::Colour;
use std::str::FromStr;
use std::error::Error;
use crate::search::Direction;
use rand::random;
use std::fs::OpenOptions;
use std::io::Write;

/// This enum denotes the possible states that any given cell on a [`Map`] can be
#[derive(Clone, Copy, PartialEq)]
pub enum CellType {
	/// The initial position, with a boolean to tag whether it's been visited or not
	Initial(bool),
	/// One of the target positions
	Target,
	/// A cell that cannot be traversed through, with a boolean to tag whether it's been visited or not
	Wall(bool),
	/// A non-special cell that can traversed through, with a boolean to tag whether it's been visited or not
	Blank(bool),
	/// Drawing component: travelling vertically
	#[cfg(feature = "eyecandy")]
	PathVer,
	/// Drawing component: travelling horizontally
	#[cfg(feature = "eyecandy")]
	PathHor,
	/// Drawing component: turn from travelling up to right
	#[cfg(feature = "eyecandy")]
	PathTopLeftCorner,
	/// Drawing component: turn from travelling up to left
	#[cfg(feature = "eyecandy")]
	PathTopRightCorner,
	/// Drawing component: turn from travelling down to right
	#[cfg(feature = "eyecandy")]
	PathBottomLeftCorner,
	/// Drawing component: turn from travelling down to left
	#[cfg(feature = "eyecandy")]
	PathBottomRightCorner,
}

/// Ascii-only rendering agent of the map for unit testing
/// (I don't really wanna put unicode in tests)
impl Debug for CellType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match *self {
			CellType::Initial(_) => "I",
			CellType::Target => "T",
			CellType::Wall(_) => "X",
			CellType::Blank(_) => " ",
			#[cfg(feature = "eyecandy")]
			x => match x {
				CellType::PathVer => "|",
				CellType::PathHor => "-",
				CellType::PathTopLeftCorner => "+",
				CellType::PathTopRightCorner => "+",
				CellType::PathBottomLeftCorner => "+",
				CellType::PathBottomRightCorner => "+",
				_ => panic!("other variants are already covered")
			}
		})
	}
}

/// Building blocks for the fancy display of the map in stderr
///
/// I had consulted https://en.wikipedia.org/wiki/Box-drawing_character for characters for drawing
#[cfg(feature = "eyecandy")]
impl Display for CellType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match *self {
			CellType::Initial(_) => Colour::Red.paint("█"),
			CellType::Target => Colour::Green.paint("█"),
			CellType::Wall(false) => Colour::White.dimmed().paint("█"),
			CellType::Blank(false) => Colour::White.dimmed().paint(" "),
			CellType::Wall(true) => Colour::Cyan.dimmed().paint("█"),
			CellType::Blank(true) => Colour::White.dimmed().on(Colour::Cyan).paint(" "),
			CellType::PathVer => Colour::Green.dimmed().on(Colour::Cyan).paint("│"),
			CellType::PathHor => Colour::Green.dimmed().on(Colour::Cyan).paint("─"),
			CellType::PathTopLeftCorner => Colour::Green.dimmed().on(Colour::Cyan).paint("┌"),
			CellType::PathTopRightCorner => Colour::Green.dimmed().on(Colour::Cyan).paint("┐"),
			CellType::PathBottomLeftCorner => Colour::Green.dimmed().on(Colour::Cyan).paint("└"),
			CellType::PathBottomRightCorner => Colour::Green.dimmed().on(Colour::Cyan).paint("┘"),
		})
	}
}

/// The Map super-object that stores all the cells in an orderly fashion, as well as provide an abstraction over the array in memory
pub struct Map<Tag> {
	/// Number of rows in the grid
	pub rows: usize,
	/// Number of cols in the grid
	pub cols: usize,
	/// Initial state
	pub initial: (usize, usize),
	/// Target states
	pub targets: Vec<(usize, usize)>,
	/// Data structure that holds the grid
	values: Vec<Tag>,
}

impl<Tag: Clone> Clone for Map<Tag> {
	fn clone(&self) -> Self {
		Map {
			rows: self.rows,
			cols: self.cols,
			initial: self.initial,
			targets: self.targets.clone(),
			values: self.values.clone(),
		}
	}
}

/// Maze parser component: turn an array (string) into an array (`Vec<number>`)
fn num_array(source: &str) -> Vec<usize> {
	source
		.trim()
		.trim_start_matches("[")
		.trim_end_matches("]")
		.trim()
		.trim_start_matches("(")
		.trim_end_matches(")")
		.trim()
		.split(",")
		.map(|x| x.trim().parse::<usize>().expect("not an unsigned number"))
		.collect()
}

/// Upper-level implementation of the maze parser
impl FromStr for Map<CellType> {
	type Err = Box<dyn Error>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut lines = s.lines();
		let (rows, cols) = {
			let grid_size = num_array(lines.next()
				.expect("missing grid size definition"));
			(grid_size[0], grid_size[1])
		};

		let mut map = Map {
			rows,
			cols,
			initial: (0, 0), // Default value, not overwritten soon
			targets: vec![],
			values: vec![CellType::Blank(false); rows * cols],
		};

		// initial cell pos
		{
			let initial = num_array(lines.next().expect("starting position required"));
			let i = map.index((initial[0], initial[1]));
			map.values[i] = CellType::Initial(false);

			map.initial = (initial[0], initial[1]);
		}

		// valid destinations pos
		{
			for target in lines.next().expect("destination position required").split("|") {
				let target = num_array(target);
				let i = map.index((target[0], target[1]));
				map.values[i] = CellType::Target;
				map.targets.push((target[0], target[1]));
			}
		}

		// walls
		{
			while let Some(wall) = lines.next() {
				let wall = num_array(wall);
				// initial coords
				let ix = wall[0];
				let iy = wall[1];

				// "delta" coords (the dimensions)
				let dx = wall[2];
				let dy = wall[3];

				for x in ix..dx + ix {
					for y in iy..dy + iy {
						let i = map.index((x, y));
						map.values[i] = CellType::Wall(false);
					}
				}
			}
		}

		Ok(map)
	}
}

impl<Tag> Map<Tag> {
	pub fn index(&self, coords: (usize, usize)) -> usize {
		coords.0 + coords.1 * self.cols
	}

	pub fn iterate<F>(&mut self, f: F) where
		F: FnMut((usize, usize), &mut Tag) {
		self.subdivision((0, 0), self.rows, self.cols, f)
	}

	/// Execute a function on a subdivision of the map
	/// where the anonymous function's coordinates are shifted by `topleft`
	/// and has the width `cols` and height `rows`
	pub fn subdivision<F>(&mut self, topleft: (usize, usize), rows: usize, cols: usize, mut f: F) where
		F: FnMut((usize, usize), &mut Tag) {
		let cx = topleft.0;
		let cy = topleft.1;

		for dy in 0..rows {
			let y = cy + dy;
			for dx in 0..cols {
				let x = cx + dx;

				f((dx, dy), self.read_cell_mut((x, y)));
			}
		}
	}

	/// Returns a reference to the cell requested
	pub fn read_cell(&self, cur: (usize, usize)) -> &Tag {
		&self.values[self.index(cur)]
	}

	/// Returns a mutable reference to the cell requested
	pub fn read_cell_mut(&mut self, cur: (usize, usize)) -> &mut Tag {
		let i = self.index(cur);
		&mut self.values[i]
	}

	/// Finds valid directions and return the associated cursor
	pub fn adjacents(&self, cur: (usize, usize)) -> Vec<(Direction, (usize, usize))> {
		Direction::iter()
			.map(|d| (*d, self.adjacent(cur, *d)))
			.filter(|x| x.1.is_some())
			.map(|x| (x.0, x.1.expect("memory corruption")))
			.collect()
	}

	/// Returns the coordinates of the adjacent cell, if none, return a [`None`]
	pub fn adjacent(&self, (x, y): (usize, usize), direction: Direction) -> Option<(usize, usize)> {
		match direction {
			Direction::Up => {
				if y > 0 {
					return Some((x, y - 1));
				}
			}
			Direction::Left => {
				if x > 0 {
					return Some((x - 1, y));
				}
			}
			Direction::Down => {
				if y < self.rows - 1 {
					return Some((x, y + 1));
				}
			}
			Direction::Right => {
				if x < self.cols - 1 {
					return Some((x + 1, y));
				}
			}
		}

		return None;
	}
}

/// Round a number into an even number, helper macro for the maze generator algorithm
macro_rules! even {
	($e:expr) => {$e / 2 * 2}
}

impl Map<CellType> {
	/// Serializes the current map and writes to a file
	pub fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
		let mut handle = OpenOptions::new()
			.create_new(true)
			.write(true)
			.open(path)?;

		writeln!(handle, "[{}, {}]", self.rows, self.cols)?;
		writeln!(handle, "({}, {})", self.initial.0, self.initial.1)?;

		let targets: Vec<String> = self.targets.iter()
			.map(|(x, y)| {
				format!("({}, {})", x, y)
			})
			.collect();

		writeln!(handle, "{}", targets.join(" | "))?;

		let mut cells = self.values.iter().enumerate();
		while let Some((idx, val)) = cells.next() {
			if let CellType::Wall(_) = val {
				writeln!(handle, "({}, {}, 1, 1)", idx % self.cols, idx / self.cols)?;
			}
		}

		Ok(())
	}

	/// Clears all the visit markers in the [`CellType`] enum
	pub fn clear_visits(&mut self) {
		self.values.iter_mut().for_each(|x| {
			if let CellType::Blank(ref mut b) | CellType::Initial(ref mut b) | CellType::Wall(ref mut b) = x {
				*b = false;
			}
		});
	}

	/// Visualize the list of directions taken by the cursor by projecting onto the map
	/// fancy paths enums in [`CellType`]
	#[cfg(feature = "eyecandy")]
	pub fn draw_path(&mut self, path: &Vec<Direction>) {
		let mut prev = None;

		let mut cursor = self.initial;
		for this in path {
			if let Some(ref prev) = prev {
				*self.read_cell_mut(cursor) = match (prev, this) {
					(Direction::Right, Direction::Up) => CellType::PathBottomRightCorner,
					(Direction::Right, Direction::Down) => CellType::PathTopRightCorner,
					(Direction::Right, _) => CellType::PathHor,
					(Direction::Left, Direction::Up) => CellType::PathBottomLeftCorner,
					(Direction::Left, Direction::Down) => CellType::PathTopLeftCorner,
					(Direction::Left, _) => CellType::PathHor,
					(Direction::Up, Direction::Left) => CellType::PathTopRightCorner,
					(Direction::Up, Direction::Right) => CellType::PathTopLeftCorner,
					(Direction::Up, _) => CellType::PathVer,
					(Direction::Down, Direction::Left) => CellType::PathBottomRightCorner,
					(Direction::Down, Direction::Right) => CellType::PathBottomLeftCorner,
					(Direction::Down, _) => CellType::PathVer,
				};
			}

			cursor = self.adjacent(cursor, *this).expect("path given is not valid");
			prev = Some(*this);
		}
	}

	/// Counts all cells that have been visited and written onto the map
	/// This is ***NOT*** the count of nodes in the search tree
	pub fn count_visited(&self) -> usize {
		self.cols * self.rows - self.values.iter()
			.filter(|x| **x == CellType::Wall(false) ||
				**x == CellType::Initial(false) ||
				**x == CellType::Blank(false)
			)
			.count()
	}

	/// Internal recursive maze generator algorithm
	/// https://en.wikipedia.org/wiki/Maze_generation_algorithm#Recursive_division_method
	fn random_maze_subdivision(&mut self, topleft: (usize, usize), rows: usize, cols: usize) {
		let victim_row = even!(random::<usize>() % (rows - 1)) + 1;
		let victim_col = even!(random::<usize>() % (cols - 1)) + 1;

		self.subdivision(topleft, rows, cols, |cur, cell| {
			if let CellType::Initial(_) = cell {
				return;
			}

			if cur.0 == victim_col || cur.1 == victim_row {
				*cell = CellType::Wall(false);
			}
		});

		let retain_wall = random::<u8>() % 4;

		// Wall 0; north
		if retain_wall != 0 {
			let row = even!(random::<usize>() % victim_row);
			*self.read_cell_mut((victim_col + topleft.0, row + topleft.1)) = CellType::Blank(false);
		}
		// Wall 2; south
		if retain_wall != 2 {
			let row = even!(random::<usize>() % (rows - victim_row) + victim_row);
			*self.read_cell_mut((victim_col + topleft.0, row + topleft.1)) = CellType::Blank(false);
		}
		// Wall 1; east
		if retain_wall != 1 {
			let col = even!(random::<usize>() % (cols - victim_col) + victim_col);
			*self.read_cell_mut((col + topleft.0, victim_row + topleft.1)) = CellType::Blank(false);
		}
		// Wall 3; west
		if retain_wall != 3 {
			let col = even!(random::<usize>() % victim_col);
			*self.read_cell_mut((col + topleft.0, victim_row + topleft.1)) = CellType::Blank(false);
		}

		if victim_row > 3 && victim_col > 3 {
			self.random_maze_subdivision((topleft.0 + 1, topleft.1 + 1), victim_row - 2, victim_col - 2);
		}

		if rows - victim_row > 3 && victim_col > 3 {
			self.random_maze_subdivision((topleft.0 + 1, victim_row + topleft.1 + 1), rows - victim_row - 2, victim_col - 2);
		}

		if cols - victim_col > 3 && victim_row > 3 {
			self.random_maze_subdivision((topleft.0 + victim_col + 1, topleft.1 + 1), victim_row - 2, cols - victim_col - 2);
		}

		if rows - victim_row > 3 && cols - victim_col > 3 {
			self.random_maze_subdivision((topleft.0 + victim_col + 1, topleft.1 + victim_row + 1), rows - victim_row - 2, cols - victim_col - 2);
		}
	}

	/// Generates a somewhat convincing maze using a recursive generation algorithm
	pub fn random_maze(rows: usize, cols: usize, targets: usize) -> Map<CellType> {
		let mut map = Map {
			rows,
			cols,
			initial: (random::<usize>() % cols, random::<usize>() % rows),
			targets: vec![],
			values: vec![CellType::Blank(false); rows * cols],
		};

		map.random_maze_subdivision((0, 0), rows, cols);

		*map.read_cell_mut(map.initial) = CellType::Initial(false);

		for _ in 0..targets {
			let coords = (random::<usize>() % cols, random::<usize>() % rows);

			map.targets.push(coords);
			*map.read_cell_mut(coords) = CellType::Target;
		}

		return map;
	}
}

/// ASCII-only renderer of a Map for unit testing
impl<X: Debug> Debug for Map<X> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for y in 0..self.rows {
			for x in 0..self.cols {
				let cur = &self.values[self.index((x, y))];

				write!(f, "{:?}", cur)?;
			}
			writeln!(f)?;
		}

		Ok(())
	}
}

/// Rich ansi-term renderer of a Map for unit testing
#[cfg(feature = "eyecandy")]
impl<X: Display> Display for Map<X> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut x_axis_border = String::new();

		for _ in 0..self.cols {
			x_axis_border += "═";
		}

		writeln!(f, "{}", Colour::White.dimmed().paint(format!("╔{}╗", x_axis_border)))?;
		for y in 0..self.rows {
			write!(f, "{}", Colour::White.dimmed().paint("║"))?;
			for x in 0..self.cols {
				let cur = &self.values[self.index((x, y))];

				write!(f, "{}", cur)?;
			}
			writeln!(f, "{}", Colour::White.dimmed().paint("║"))?;
		}

		writeln!(f, "{}", Colour::White.dimmed().paint(format!("╚{}╝", x_axis_border)))?;

		Ok(())
	}
}