use crate::{Config, runner, SearchReport};
use crate::map::Map;
use std::{thread};
use std::marker::PhantomData;
use std::time::SystemTime;

const THREADS: usize = 12;
const RUNS_PER_THREAD: usize = 5_000_000;
const METHODS: &'static [&'static str; 6] = &["BFS", "DFS", "GBFS", "AS", "CUS1", "CUS2"];

pub fn discrete_fuzzing_unit(config: &Config) {
	let map = {
		let (rows, cols) = config.rand_size();
		Map::random_maze(rows, cols, config.target_count())
	};

	let result: Vec<(SearchReport, _)> = METHODS.iter()
		.map(|method| {
			let mut map = map.clone();
			let mut config = config.clone();
			config.method = method.to_string();

			let past = SystemTime::now();
			let res = runner(&mut map, &config).unwrap();

			let elapsed_ns = SystemTime::now().duration_since(past).unwrap().as_nanos();
			(res, elapsed_ns)
		})
		.collect();

	result.iter()
		.map(|(x, _)| x.solution.is_some())
		.fold_first(|acc, x| {
			if acc != x {
				eprintln!("Disagreement on whether there was a solution on map:");
				eprintln!("{}", map);
			}
			acc
		});

	let timings: Vec<_> = result.iter()
		.map(|(_, x)| format!("{}", x))
		.collect();
	print!("{}, ", timings.join(", "));

	let nodes: Vec<_> = result.iter()
		.map(|(x, _)| x.search_nodes)
		.map(|x| format!("{}", x))
		.collect();
	print!("{}, ", nodes.join(", "));

	let path_lens: Vec<_> = result.into_iter()
		.map(|(x, _)| x.solution)
		.filter(|x| x.is_some())
		.map(|x| x.unwrap_or(vec![]))
		.map(|x| format!("{}", x.len()))
		.collect();
	println!("{}", path_lens.join(", "));
}

pub fn wrapper(config: Config) {
	print!("{}, ", METHODS.join(", "));
	print!("{}, ", METHODS.join(", "));
	println!("{}", METHODS.join(", "));
	let _: Vec<_> = vec![PhantomData; THREADS].into_iter()
		.map(|_: PhantomData<()>| config.clone())
		.map(|config| {
			thread::spawn(move || {
				for _ in 0..RUNS_PER_THREAD {
					discrete_fuzzing_unit(&config);
				}
			})
		})
		.map(|x| x.join())
		.collect();
}

