#!/bin/sh
MODES="DFS GBFS ASTAR DIJKSTRA WASTAR"

cargo build --release --no-default-features
cp target/release/assignment_1 target/release/fuzzer_runme

mkdir random_maps

while true; do
  FILE_NAME=$(date "+random_maps/%Y-%m-%dT%k%M%S.%N.txt")
  echo "========================================================="
  echo "File is $FILE_NAME"
  echo
  target/release/fuzzer_runme --save-map $FILE_NAME RANDOM BFS
  for MODE in $MODES; do
    echo
    target/release/fuzzer_runme $FILE_NAME $MODE
  done
  echo
  echo "========================================================="
done
