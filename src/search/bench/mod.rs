use test::Bencher;
use crate::Map;
use crate::utils::filter;
use crate::search::graph_search::GraphSearch;
use crate::utils::queue::sorted::SortedQueue;
use crate::{AStarCursor, DijkstraCursor, GBFCursor, ManhattanHeuristic, UniformMoveWeight, WeightedASCursor};
use crate::search::Search;
use std::mem;
use crate::search::bfs::BreadthFirst;
use crate::search::dfs::DepthFirst;
use crate::search::iddfs::IterativeDeepening;

const STATIC_MAP: &'static str = include_str!("map.txt");

#[bench]
fn a_star(b: &mut Bencher) {
	b.iter(|| {
		let mut map = Map::random_maze(50, 50, 2);
		let mut search = graph_search!(&mut map, ManhattanHeuristic::init(&map), AStarCursor<UniformMoveWeight>);

		let _ = search.search();

		mem::drop(search);

		map.clear_visits();
	})
}

#[bench]
fn breadth_first(b: &mut Bencher) {
	b.iter(|| {
		let mut map = Map::random_maze(50, 50, 2);
		let mut search = BreadthFirst::init(&mut map);

		let _ = search.search();

		mem::drop(search);

		map.clear_visits();
	})
}

#[bench]
fn depth_first(b: &mut Bencher) {
	b.iter(|| {
		let mut map = Map::random_maze(50, 50, 2);
		let mut search = DepthFirst::init(&mut map);

		let _ = search.search();

		mem::drop(search);

		map.clear_visits();
	})
}

#[bench]
fn iddfs(b: &mut Bencher) {
	b.iter(|| {
		let mut map = Map::random_maze(50, 50, 2);
		let mut search = IterativeDeepening::init(&mut map);

		let _ = search.search();

		mem::drop(search);

		map.clear_visits();
	})
}

#[bench]
fn dijkstra(b: &mut Bencher) {
	b.iter(|| {
		let mut map = Map::random_maze(50, 50, 2);
		let mut search = graph_search!(&mut map, ManhattanHeuristic::init(&map), DijkstraCursor<UniformMoveWeight>);

		let _ = search.search();

		mem::drop(search);

		map.clear_visits();
	})
}

#[bench]
fn greedy_best_first(b: &mut Bencher) {
	b.iter(|| {
		let mut map = Map::random_maze(50, 50, 2);
		let mut search = graph_search!(&mut map, ManhattanHeuristic::init(&map), GBFCursor<UniformMoveWeight>);

		let _ = search.search();

		mem::drop(search);

		map.clear_visits();
	})
}

#[bench]
fn weighted_astar(b: &mut Bencher) {
	b.iter(|| {
		let mut map = Map::random_maze(50, 50, 2);
		let mut search = graph_search!(&mut map, ManhattanHeuristic::init(&map), WeightedASCursor<UniformMoveWeight>);

		let _ = search.search();

		mem::drop(search);

		map.clear_visits();
	})
}

