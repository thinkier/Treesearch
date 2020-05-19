use crate::search::Search;
use crate::map::{Map, CellType};
use crate::SearchReport;
use crate::search::dfs::SearchResult;

pub struct IterativeDeepening<'a> {
	map: &'a mut Map<CellType>,
}

impl<'a> Search for IterativeDeepening<'a> {
	fn search(&mut self) -> SearchReport {
		let mut limit = 0;
		let mut map = self.map.to_owned();
		loop {
			let (result, more) = self.recurse(self.map.initial, limit);

			match result {
				SearchResult::Hit(count, mut res) => {
					res.reverse();

					// Clean up the output if need be
					#[cfg(feature = "eyecandy")]
						map.iterate(|(x, y), old| {
						match old {
							CellType::Initial(old_cv) |
							CellType::Blank(old_cv) |
							CellType::Wall(old_cv) => {
								if !*old_cv {
									return;
								}
							}
							_ => return
						}

						match self.map.read_cell_mut((x, y)) {
							CellType::Initial(ref mut cv) |
							CellType::Blank(ref mut cv) |
							CellType::Wall(ref mut cv) => {
								*cv = true
							}
							_ => return
						}
					});

					return SearchReport {
						search_nodes: count,
						solution: Some(res),
					};
				}
				SearchResult::Miss(count) => {
					if more {
						limit += 1;

						map = self.map.to_owned();
						self.map.clear_visits();
					} else {
						return SearchReport {
							search_nodes: count,
							solution: None,
						};
					}
				}
			}
		}
	}
}

impl<'a> IterativeDeepening<'a> {
	pub fn init(map: &mut Map<CellType>) -> IterativeDeepening {
		IterativeDeepening { map }
	}

	/// The "deepening" part of "Iterative Deepening"
	fn recurse(&mut self, cur: (usize, usize), lim: usize) -> (SearchResult, bool) {
		let mut count = 1;
		match self.map.read_cell_mut(cur) {
			CellType::Target => return (SearchResult::Hit(count, vec![]), false),
			CellType::Wall(ref mut cv) => {
				*cv = true;
			}
			// If it's the initial cell or a blank cell
			CellType::Initial(ref mut cv) |
			CellType::Blank(ref mut cv) => {
				if lim == 0 {
					return (SearchResult::Miss(count), !*cv);
				}

				if *cv { return (SearchResult::Miss(count), false); }
				*cv = true; // Cosmetic visited

				let mut inner_has_more = false;
				for (dir, pos) in self.map.adjacents(cur) {
					let (inner, more) = self.recurse(pos, lim - 1);

					match &inner {
						SearchResult::Hit(inner_count, _) | SearchResult::Miss(inner_count) =>
							count += *inner_count
					}

					if let SearchResult::Hit(_, mut path) = inner {
						// Added to vector in reverse.
						path.push(dir); // Alternative (shift on) would mean lots of reallocs
						return (SearchResult::Hit(count, path), more);
					}

					inner_has_more |= more;
				}

				return (SearchResult::Miss(count), inner_has_more);
			}
			#[cfg(feature = "eyecandy")]
			_ => {}
		};
		return (SearchResult::Miss(count), false);
	}
}
