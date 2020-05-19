mod parse {
	use crate::map::{Map, CellType};
	use std::str::FromStr;
	use std::fmt::Debug;

	#[test]
	fn default_file_parse() {
		let map: Map<CellType> = Map::from_str(include_str!("RobotNav-test.txt")).unwrap();
		parse_test(&map);
	}

	#[test]
	fn messy_space_file_parse() {
		let map: Map<CellType> = Map::from_str(include_str!("RobotNav-test_MESSY_SPACE.txt")).unwrap();
		parse_test(&map);
	}

	fn parse_test<T: Debug + Clone>(map: &Map<T>) {
		format!("{:?}", map).lines().zip([
			"  XX   TX X",
			"I XX    X  ",
			"           ",
			"  X      XT",
			"  XXXX  XX ",
		].iter())
			.for_each(|(gen, expect)| {
				assert_eq!(gen, *expect);
			});
	}
}