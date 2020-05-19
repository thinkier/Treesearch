# Treesearch

## Basic Usage Guide
```
cargo run [options] <map file here> <method>
```

Where `[options]` is one or more of the following:
- `--variable-move-weight` - Enables dynamic move weights to penalize moving in different directions.
- `--map-size <rows> <cols>` - Allows the randomly generated map to have a fixed size instead of having that randomised as well.
- `--targets <target count>` - Sets the number of goal/target positions available on the randomly generated map.
- `--save-map <location>` - Reserialize the current map to `location`, handy for testing randomly generated maps.

Where `<method>` is one of the following:
- `BFS`
- `DFS`
- `GBFS`
- `AS` (or `ASTAR`)
- `CUS1` (or `IDDFS`)
- `CUS2` (or `WASTAR`)

Map file can be replaced with `RANDOM` to have a mediocre random maze generated. Please consult `src/map` for example map files.

## General Notes
This was made for a university assignment involving implementing search algorithms for an agent. The assignment is marked out of 110 marks, of which 108 were obtained. The reason 2 marks were lost were that the terminal maze visualizer that was "confusing" to the assessor.
