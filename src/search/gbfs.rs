use crate::search::{Direction, Cursor, MoveWeight};
use crate::utils::queue::sorted::Weighted;

#[derive(Default, Debug)]
pub struct GBFCursor<W> {
	weight: usize,
	path: Vec<Direction>,
	cursor: (usize, usize),
	weigher: W,
}

impl<W> Cursor for GBFCursor<W> where
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

impl<W> Weighted for GBFCursor<W> {
	/// GBFS weight = heuristic weight
	fn weigh(&self) -> usize {
		self.weight
	}

	fn heuristic_weight(&mut self) -> &mut usize {
		&mut self.weight
	}

	fn direction(&self) -> Option<Direction> {
		self.path.last().map(|x| *x)
	}
}
