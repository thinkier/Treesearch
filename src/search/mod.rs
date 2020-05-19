pub mod dfs;
pub mod bfs;
pub mod gbfs;
pub mod astar;
pub mod dijkstra;
pub mod iddfs;
pub mod wastar;
pub mod graph_search;

#[cfg(test)]
mod bench;

use std::fmt::{Display, self};
use std::slice::Iter;
use crate::SearchReport;

/// Interface to define the basic functionality of a search algorithm
pub trait Search {
	fn search(&mut self) -> SearchReport;
}

/// List of possible directions to take for the intelligent agent
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Direction {
	Up = 0,
	Left = 1,
	Down = 2,
	Right = 3,
}

/// Convert the enum into human friendly text representation
impl Display for Direction {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Direction::Up => write!(f, "Up"),
			Direction::Left => write!(f, "Left"),
			Direction::Down => write!(f, "Down"),
			Direction::Right => write!(f, "Right"),
		}
	}
}

impl Direction {
	/// Create an iterator of [`Direction`]s.
	/// This is specified in the assignment document as up, left, down, right, in that order
	pub fn iter() -> Iter<'static, Direction> {
		[Direction::Up, Direction::Left, Direction::Down, Direction::Right].iter()
	}
}

/// Experiment: Vary the weights of travelling in different directions to see
pub trait MoveWeight: Default {
	fn weigh(&self, d: &Direction) -> usize;
}

/// Simple uniform-weight direction weigher
#[derive(Default)]
pub struct UniformMoveWeight;

impl MoveWeight for UniformMoveWeight {
	fn weigh(&self, _d: &Direction) -> usize {
		1
	}
}

/// Weight varies when moving in different directions, as suggested for research initiative
#[derive(Default)]
pub struct CustomMoveWeight;

impl CustomMoveWeight {
	pub fn weigh(d: &Direction) -> usize {
		match *d {
			Direction::Up => 4,
			Direction::Down => 1,
			Direction::Left => 3,
			Direction::Right => 2
		}
	}
}

impl MoveWeight for CustomMoveWeight {
	fn weigh(&self, d: &Direction) -> usize {
		CustomMoveWeight::weigh(d)
	}
}

/// Define the basic functionality that an abstract Cursor object should have,
/// for interoperabiltiy of different searches of the common implementation of graph-based search
pub trait Cursor {
	type DirectionWeigher: MoveWeight;

	fn path(&self) -> &Vec<Direction>;
	fn path_mut(&mut self) -> &mut Vec<Direction>;
	fn cursor(&self) -> &(usize, usize);
	fn cursor_mut(&mut self) -> &mut (usize, usize);
	fn into_path(self) -> Vec<Direction>;
}