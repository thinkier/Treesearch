#![feature(test)]
#![feature(iterator_fold_self)]

//! This is my Assignment 1 code for COS20019 - Introduction to Artificial Intelligence
//!
//! In the codebase below, I implemented various search algorithms as requested in the assignment specifications.
//! For basic usage, please consult README.md

#[cfg(test)]
extern crate test;
#[cfg(feature = "eyecandy")]
extern crate ansi_term;
extern crate rand;

use crate::map::{Map, CellType};
use std::error::Error;
use std::env;
use rand::random;
use crate::search::dfs::DepthFirst;
use crate::search::bfs::BreadthFirst;
use crate::search::{Search, UniformMoveWeight, CustomMoveWeight, Direction};
use crate::search::gbfs::GBFCursor;
use crate::utils::heuristics::{DefaultHeuristic, CustomManhattan, ManhattanHeuristic};
use crate::utils::queue::sorted::SortedQueue;
use crate::search::graph_search::GraphSearch;
use crate::search::astar::AStarCursor;
use crate::search::wastar::WeightedASCursor;
use crate::utils::filter;
use crate::search::dijkstra::DijkstraCursor;

#[cfg(not(feature = "fuzzing"))]
use std::{fs, mem, str::FromStr};
use crate::search::iddfs::IterativeDeepening;

#[cfg(feature = "fuzzing")]
mod fuzzing;

macro_rules! graph_search {
	($map:expr, $heu:expr, $cursor:ty) => {{
		let heu = $heu;

		Box::new(GraphSearch::init(
			$map,
			heu,
			SortedQueue::init::<$cursor>(),
			filter::global_duped,
		))
	}}
}

mod map;
mod search;
mod utils;

#[derive(Clone, Default)]
pub struct Config {
	// The input map file, can be RANDOM
	pub map_file: String,
	// Search method, refer to docs
	pub method: String,
	// Where to copy the map to as a save, useful for testing with random
	pub save_map: Option<String>,
	// Variable move weight experiment (ie. the weights of moving in different direction changes)
	pub var_move_wt: bool,
	// The follow relates to randomly generated maps
	pub rand_size: Option<(usize, usize)>,
	pub target_count: Option<usize>,
}

impl Config {
	fn rand_size(&self) -> (usize, usize) {
		self.rand_size.clone().unwrap_or((random::<usize>() % 16 + 16, random::<usize>() % 112 + 16))
	}

	fn target_count(&self) -> usize {
		self.target_count.clone().unwrap_or(2)
	}
}

/// This is the application entry point, it accepts arguments in the form of `executable <maze_file> <search algorithm>`
fn main() -> Result<(), Box<dyn Error>> {
	let mut args = env::args().skip(1).peekable();

	let mut config = Config::default();

	while let Some(arg) = args.peek() {
		match arg.to_lowercase().as_ref() {
			"--variable-move-weight" => config.var_move_wt = true,
			"--map-size" => {
				args.next().unwrap(); // Advance the iterator since we've peeked above

				let rows = args.next()
					.expect("please specify the number of rows after --map-size")
					.parse::<usize>()
					.expect("the rows count is not a number");
				let cols = args.peek()
					.expect("please specify the number of columns after --map-size")
					.parse::<usize>()
					.expect("the columns count is not a number");
				config.rand_size = Some((rows, cols))
			}
			"--targets" => {
				args.next().unwrap(); // Advance the iterator since we've peeked above

				let count = args.peek()
					.expect("please specify the number of targets after --targets")
					.parse::<usize>()
					.expect("the targets count is not a number");
				config.target_count = Some(count)
			}
			"--save-map" => {
				args.next().unwrap();
				config.save_map = Some(args.peek().expect("please specify the file name to save the map to").to_owned())
			}
			_ => break
		}
		args.next().unwrap(); // move forward since we were peeking
	}

	#[cfg(feature = "fuzzing")] {
		fuzzing::wrapper(config);
	}

	#[cfg(not(feature = "fuzzing"))]
		{
			config.map_file = args.next().expect("test file required");
			config.method = args.next().expect("test method required").to_ascii_uppercase();
			let mut map: Map<CellType> = if config.map_file.trim() == "RANDOM" {
				let (rows, cols) = config.rand_size();
				Map::random_maze(rows, cols, config.target_count())
			} else {
				Map::from_str(
					&fs::read_to_string(&config.map_file)
						.expect("test file cannot be found")
				)
					.expect("failed to parse test file")
			};

			if let Some(path) = &config.save_map {
				map.save(path).expect("failed to copy map");
			}

			let _ = runner(&mut map, &config)?;
		}
	return Ok(());
}

pub struct SearchReport {
	pub search_nodes: usize,
	pub solution: Option<Vec<Direction>>,
}

pub fn runner(map: &mut Map<CellType>, config: &Config) -> Result<SearchReport, Box<dyn Error>> {
	let mut implementation: Box<dyn Search> = match config.method.to_ascii_uppercase().as_ref() {
		"DFS" => Box::new(DepthFirst::init(map)),
		"BFS" => Box::new(BreadthFirst::init(map)),
		"GBFS" => {
			if config.var_move_wt {
				graph_search!(map, CustomManhattan::init(&map), GBFCursor<CustomMoveWeight>)
			} else {
				graph_search!(map, ManhattanHeuristic::init(&map), GBFCursor<UniformMoveWeight>)
			}
		}
		"AS" | "ASTAR" => {
			if config.var_move_wt {
				graph_search!(map, CustomManhattan::init(&map), AStarCursor<CustomMoveWeight>)
			} else {
				graph_search!(map, ManhattanHeuristic::init(&map), AStarCursor<UniformMoveWeight>)
			}
		}
		"CUS1" | "IDDFS" => {
			Box::new(IterativeDeepening::init(map))
		}
		"CUS2" | "WAS" | "WASTAR" | "WEIGHTED_ASTAR" => {
			if config.var_move_wt {
				graph_search!(map, CustomManhattan::init(&map), WeightedASCursor<CustomMoveWeight>)
			} else {
				graph_search!(map, ManhattanHeuristic::init(&map), WeightedASCursor<UniformMoveWeight>)
			}
		}
		"UCS" | "DIJKSTRA" => {
			if config.var_move_wt {
				graph_search!(map, DefaultHeuristic::default(), DijkstraCursor<CustomMoveWeight>)
			} else {
				graph_search!(map, DefaultHeuristic::default(), DijkstraCursor<UniformMoveWeight>)
			}
		}
		x => {
			panic!("unrecognized search method: {}", x)
		}
	};

	let report = implementation.search();

	#[cfg(not(feature = "fuzzing"))] {
		mem::drop(implementation);

		println!("{} {} {}", config.map_file, config.method, report.search_nodes);
		if let Some(path) = &report.solution {
			println!("{}", path.iter()
				.map(|d| format!("{}; ", d))
				.fold(String::new(), |x, d| {
					x + &d
				})
				.trim()
			);

			#[cfg(feature = "eyecandy")]
				map.draw_path(&path);
		} else {
			println!("No solution found.");
		}

		#[cfg(feature = "eyecandy")]
		eprintln!("{}", map);
	}
	Ok(report)
}
