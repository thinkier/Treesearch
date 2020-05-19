use crate::search::{Search, Direction};
use crate::map::{CellType, Map};
use crate::SearchReport;

pub enum SearchResult {
	Hit(usize, Vec<Direction>),
	Miss(usize),
}

pub struct DepthFirst<'a> {
	map: &'a mut Map<CellType>
}

impl<'a> Search for DepthFirst<'a> {
	fn search(&mut self) -> SearchReport {
		let path = self.recurse(self.map.initial);

		return match path {
			SearchResult::Miss(count) => {
				SearchReport { search_nodes: count, solution: None }
			}
			SearchResult::Hit(count, mut path) => {
				// As it recurses it does not shift elements on so it has to be reversed
				path.reverse();
				SearchReport { search_nodes: count, solution: Some(path) }
			}
		};
	}
}

impl<'a> DepthFirst<'a> {
	pub fn init(map: &mut Map<CellType>) -> DepthFirst {
		DepthFirst { map }
	}

	/// Implementation of depth-first, where all the travel history are stored in stack
	fn recurse(&mut self, cur: (usize, usize)) -> SearchResult {
		let mut count = 1;
		match self.map.read_cell_mut(cur) {
			CellType::Target => return SearchResult::Hit(count, vec![]),
			CellType::Wall(ref mut visited) => *visited = true,
			// If it's the initial cell or a blank cell
			CellType::Initial(ref mut visited) |
			CellType::Blank(ref mut visited) => {
				// Mark current cell or skip if it's marked already
				if *visited { return SearchResult::Miss(1); }
				*visited = true;

				for (dir, pos) in self.map.adjacents(cur) {
					let inner = self.recurse(pos);

					match &inner {
						SearchResult::Hit(inner_count, _) | SearchResult::Miss(inner_count) =>
							count += *inner_count
					}

					if let SearchResult::Hit(_, mut path) = inner {
						// Added to vector in reverse.
						path.push(dir); // Alternative (shift on) would mean lots of reallocs
						return SearchResult::Hit(count, path);
					}
				}

				return SearchResult::Miss(count);
			}
			#[cfg(feature = "eyecandy")]
			_ => {}
		};
		return SearchResult::Miss(count);
	}
}