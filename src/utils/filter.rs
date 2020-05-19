use crate::search::{Cursor, Direction};
use crate::map::{Map, CellType};

/// Parse the cursor's path list to detect if the cursor has looped back on its path
#[allow(unused)]
pub fn branch_duped<C>(_map: &Map<CellType>, cur: &C) -> bool where
	C: Cursor {
	let path = cur.path();

	let (mut lat_dist, mut long_dist) = (0, 0);

	let len = path.len();
	for back in 1..len + 1 {
		let i = len - back;

		match path[i] {
			Direction::Up => long_dist -= 1,
			Direction::Down => long_dist += 1,
			Direction::Left => lat_dist -= 1,
			Direction::Right => lat_dist += 1,
		}

		if lat_dist == 0 && long_dist == 0 {
			return true;
		}
	}

	return false;
}

/// Check against the global map if the current cell has been visited
pub fn global_duped<C>(map: &Map<CellType>, cur: &C) -> bool where
	C: Cursor {
	match map.read_cell(*cur.cursor()) {
		CellType::Initial(visited) | CellType::Blank(visited) | CellType::Wall(visited) => *visited,
		_ => false
	}
}
