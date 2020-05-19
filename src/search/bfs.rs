use crate::search::{Search, Direction};
use crate::map::{Map, CellType};
use crate::utils::queue::fifo::FIFOQueue;
use crate::utils::queue::QueueStrategy;
use crate::SearchReport;

pub struct BreadthFirst<'a> {
	map: &'a mut Map<CellType>
}

impl<'a> Search for BreadthFirst<'a> {
	/// It's a tree based search with the parents enumerated by the cursor
	/// and the state repetitions written onto the map directly
	fn search(&mut self) -> SearchReport {
		// This creates a FIFO queue for sending [`Cursor`]s around; the "frontier"
		let mut queue = FIFOQueue::init();

		// Kickstart the search
		queue.queue(BFSCursor { path: vec![], cursor: self.map.initial });

		let mut i = 0;

		while let Some(cur) = queue.dequeue() {
			i += 1;
			// Mark current cell
			match self.map.read_cell_mut(cur.cursor) {
				// If it's a cell that's not a path or target
				CellType::Initial(ref mut visited) |
				CellType::Blank(ref mut visited) |
				CellType::Wall(ref mut visited) => if *visited {
					continue; // This removes repeated states
					// Identical states on alternative branches are also pruned
					// because it's a lot more work to do the checking, and waste of effort anyway
					// since this is uninformed search
				} else {
					*visited = true;
				}
				CellType::Target => {
					return SearchReport { search_nodes: i, solution: Some(cur.path) };
				}
				#[cfg(feature = "eyecandy")]
				_ => continue
			}

			self.expose_next_layer(cur, &mut queue);
		}

		return SearchReport { search_nodes: i, solution: None };
	}
}

/// Basic cursor since BFS does not use the stack to store its state
#[derive(Debug)]
pub struct BFSCursor {
	pub path: Vec<Direction>,
	pub cursor: (usize, usize),
}

impl<'a> BreadthFirst<'a> {
	pub fn init(map: &mut Map<CellType>) -> BreadthFirst {
		BreadthFirst { map }
	}

	/// Takes the sender of the FIFO queue and attach all children to it
	/// But only if it's a blank cell, otherwise it's ignored
	fn expose_next_layer(&mut self, cur: BFSCursor, queue: &mut FIFOQueue<BFSCursor>) {
		match self.map.read_cell(cur.cursor) {
			CellType::Initial(_) | CellType::Blank(_) => { // Unvisited blank, add neighbours to queue
				for (dir, pos) in self.map.adjacents(cur.cursor) {
					let mut path = cur.path.clone();
					path.push(dir);

					queue.queue(BFSCursor { path, cursor: pos });
				}
			}
			// Walls, Paths etc. are just left alone
			_ => {}
		}
	}
}