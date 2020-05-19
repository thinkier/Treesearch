use crate::map::Map;
use test::Bencher;

#[bench]
fn map_gen_8sq(b: &mut Bencher) {
	b.iter(|| {
		Map::random_maze(8, 8, 2)
	});
}

#[bench]
fn map_gen_16sq(b: &mut Bencher) {
	b.iter(|| {
		Map::random_maze(16, 16, 2)
	});
}


#[bench]
fn map_gen_50sq(b: &mut Bencher) {
	b.iter(|| {
		Map::random_maze(50, 50, 2)
	});
}

#[bench]
fn map_gen_console_max(b: &mut Bencher) {
	b.iter(|| {
		Map::random_maze(127, 31, 2)
	});
}
