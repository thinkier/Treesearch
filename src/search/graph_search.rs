use crate::map::{CellType, Map};
use crate::utils::queue::QueueStrategy;
use std::marker::PhantomData;
use crate::utils::queue::sorted::Weighted;
use crate::utils::heuristics::Heuristic;
use crate::search::{Cursor, Search};
use crate::SearchReport;

/// Abstract implementation for a graph search strategy
///
/// - Where it can have a custom heuristic function
/// - Where it can have a custom queueing strategy
/// 	- By extension requiring a custom cursor
/// - Where it can have a custom duplication checking strategy
pub struct GraphSearch<'a, H, Q, C> {
	map: &'a mut Map<CellType>,
	heuristic: H,
	queue: Q,
	_cursor: PhantomData<C>,
	filter: fn(&Map<CellType>, &C) -> bool,
}

impl<'a, H, Q, C> Search for GraphSearch<'a, H, Q, C> where
	H: Heuristic,
	C: Cursor + Weighted + Default,
	Q: QueueStrategy<C>, {
	fn search(&mut self) -> SearchReport {
		// Kickstart the search
		{
			let mut initial = C::default();

			// Give cursor available information
			*initial.heuristic_weight() = self.heuristic.estimate(self.map.initial);
			*initial.cursor_mut() = self.map.initial;

			// Lazily add to queue
			self.queue.queue(initial);
		}

		let mut count = 0;
		while let Some(cur) = self.queue.dequeue() {
			count += 1;

			// Use the duplication checking strategy to verify the cell's validity
			let skip = (self.filter)(self.map, &cur);

			// Mark current cell
			match self.map.read_cell_mut(*cur.cursor()) {
				// If it's a cell that's not a path or target
				CellType::Initial(ref mut visited) |
				CellType::Blank(ref mut visited) |
				CellType::Wall(ref mut visited) => {
					if skip {
						continue;
					}

					*visited = true;
				}
				CellType::Target => {
					return SearchReport { search_nodes: count, solution: Some(cur.into_path()) };
				}
				#[cfg(feature = "eyecandy")]
				_ => continue
			}

			// If all else goes well, expand this cell's children
			self.expose_next_layer(cur);
		}

		return SearchReport {
			search_nodes: count,
			solution: None,
		};
	}
}

impl<'a, H, Q, C> GraphSearch<'a, H, Q, C> where
	H: Heuristic,
	C: Cursor + Weighted + Default,
	Q: QueueStrategy<C> {
	pub fn init(map: &mut Map<CellType>, h: H, q: Q, filter: fn(&Map<CellType>, &C) -> bool) -> GraphSearch<H, Q, C> {
		GraphSearch { map, heuristic: h, queue: q, _cursor: PhantomData, filter }
	}

	fn expose_next_layer(&mut self, mut cur: C) {
		match self.map.read_cell(*cur.cursor()) {
			CellType::Initial(_) | CellType::Blank(_) => { // Unvisited blank, add neighbours to queue
				for (dir, pos) in self.map.adjacents(*cur.cursor()) {
					let mut neighbour = C::default();

					*neighbour.heuristic_weight() = self.heuristic.estimate(pos);
					*neighbour.cursor_mut() = pos;
					neighbour.path_mut().extend_from_slice(&cur.path_mut());
					neighbour.path_mut().push(dir);

					self.queue.queue(neighbour);
				}
			}
			// Walls, Paths etc. are just left alone
			_ => {}
		}
	}
}
