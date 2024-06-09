#!/usr/bin/env bash

cargo run -- -f examples/demo.hanb > tests/example_output.txt
cargo run -- -f examples/demo2.hanb
mv hanb.hsit examples/example.hsit
cat tests/example_output.txt
cat examples/example.hsit
