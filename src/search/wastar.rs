use crate::search::{Direction, Cursor, MoveWeight};
use crate::utils::queue::sorted::Weighted;

const WEIGHT_MODIFIER: usize = 2;

#[derive(Default, Debug)]
pub struct WeightedASCursor<W> {
	heuristic_weight: usize,
	path: Vec<Direction>,
	cursor: (usize, usize),
	weigher: W,
}

impl<W> Cursor for WeightedASCursor<W> where
	W: MoveWeight {
	type DirectionWeigher = W;

	fn path(&self) -> &Vec<Direction> {
		&self.path
	}

	fn path_mut(&mut self) -> &mut Vec<Direction> {
		&mut self.path
	}

	fn cursor(&self) -> &(usize, usize) {
		&self.cursor
	}

	fn cursor_mut(&mut self) -> &mut (usize, usize) {
		&mut self.cursor
	}

	fn into_path(self) -> Vec<Direction> {
		self.path
	}
}

impl<W> Weighted for WeightedASCursor<W> where
	W: MoveWeight {
	/// Weighted AStar weight = travelled weight + (weighing function * heuristic weight)
	/// In this case, the weighing function is also heuristic weight
	fn weigh(&self) -> usize {
		self.heuristic_weight * WEIGHT_MODIFIER + self.path.iter()
			.map(|d| self.weigher.weigh(d))
			.fold(0, |fold, add| fold + add)
	}

	fn heuristic_weight(&mut self) -> &mut usize {
		&mut self.heuristic_weight
	}

	fn direction(&self) -> Option<Direction> {
		self.path.last().map(|x| *x)
	}
}
