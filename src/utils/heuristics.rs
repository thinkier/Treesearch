use crate::map::{CellType, Map};
use crate::search::{CustomMoveWeight, Direction};
use std::ops::Mul;

/// Trait for a basic heuristic function, universal adapters are good
pub trait Heuristic {
	fn estimate(&self, cursor: (usize, usize)) -> usize;
}

/// A heuristic adapter that always returns 0 for compatibility with the existing code
#[derive(Default)]
pub struct DefaultHeuristic;

impl Heuristic for DefaultHeuristic {
	fn estimate(&self, _cursor: (usize, usize)) -> usize {
		0
	}
}

/// Manhattan-distance heuristic function, chose because it's nice and easy to implement
// Not necessarily the best heuristic function so I will add more later probably
#[derive(Default)]
pub struct ManhattanHeuristic {
	targets: Vec<(usize, usize)>,
}

impl ManhattanHeuristic {
	pub fn init(map: &Map<CellType>) -> Self {
		ManhattanHeuristic {
			targets: map.targets.to_owned(),
		}
	}
}

impl Heuristic for ManhattanHeuristic {
	fn estimate(&self, cursor: (usize, usize)) -> usize {
		self.targets.iter()
			.map(|(ref x, ref y)| {
				// Sum of differences between cursor and current target on both axes
				(cursor.0.max(*x) - cursor.0.min(*x)) + (cursor.1.max(*y) - cursor.1.min(*y))
			})
			.min() // Lead to the closer target
			.expect("no targets found")
	}
}

/// Manhattan-distance heuristic function, but adjusted such that it takes the variable-distance effects of [`VariableMoveWeight`]
#[derive(Default)]
pub struct CustomManhattan {
	targets: Vec<(usize, usize)>,
}

impl CustomManhattan {
	pub fn init(map: &Map<CellType>) -> Self {
		CustomManhattan {
			targets: map.targets.to_owned(),
		}
	}
}

impl Heuristic for CustomManhattan {
	fn estimate(&self, cursor: (usize, usize)) -> usize {
		self.targets.iter()
			.map(|(ref x, ref y)| {
				// Sum of differences between cursor and current target on both axes
				let (mut x, mut y) = (*x as isize, *y as isize);

				let target_x = cursor.0 as isize;
				let target_y = cursor.1 as isize;

				x -= target_x;
				y -= target_y;

				let x_weight = if x < 0 {
					CustomMoveWeight::weigh(&Direction::Right) * x.mul(-1) as usize
				} else {
					CustomMoveWeight::weigh(&Direction::Left) * x as usize
				};

				let y_weight = if y < 0 {
					CustomMoveWeight::weigh(&Direction::Down) * y.mul(-1) as usize
				} else {
					CustomMoveWeight::weigh(&Direction::Up) * y as usize
				};

				x_weight + y_weight
			})
			.min() // Lead to the closer target
			.expect("no targets found")
	}
}
