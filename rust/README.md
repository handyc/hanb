# Rust Implementation of Hanb

## Clone, Build, Run

You must have rust and cargo installed.

* Clone this repo:

```sh
git clone https://github.com/handyc/hanb.git
```

* cd into rust

```sh
cd rust
```

* Build

```sh
cargo build --release
```

* Run

```sh
./target/release/hanb xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

Alternatively
```sh
# Alter
cargo run -- AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA                              <<<
```

## Launch the repl
```
hanb
```

Then type `help`.

```
Hanb help:
  help | h  [command: string = None] -> Prints this help message
  print | p  -> Prints the current board
  printseq | ps  -> Prints the current board as a string sequence
  up | u  -> Move up one level
  down | d  [cell: int = 0] -> Move down one level
  board | b  [board: string = .] -> Set the current board
  save | s  [filename: string = hanb.hsit] -> Saves the current explored map to a file
  load | l  [filename: string = hanb.hsit] [Level: string = .] -> Load a saved explored map from a file
  export | e  [script: string = script.hanb] -> Export the current repl history as a hanb script
  import | i  [script: string = script.hanb] -> Import a hanb script and execute it into this repl overwriting t
he current navigator
Arguments are represented like: [name: type = default]
```

Notice you can get help for specific commands.

## Use in shell scripts.

```
echo -e "aa\$\$\$%**no*)(=aaaaaaaaaaaaaaaaaaaaaaaaaaaaa;aaaaa aaa\np" | hanb -s
```


## Hanb scripts
Hanb supports a scripting language. The idea is navigating through the boards and setting the cell sizes. You can list the supported commands with `hanb -c`.

Check the examples folder. Run with:

```sh
cargo run -- -f examples/demo.hanb
```

That file contains some info and instructions about the query language.

## Hanb explored maps
Hanb universes can be serialized to explored maps. Check `examples/example.hsit`. You can save explored maps from the repl with `save` and load one with `load`. They store the state of the entire universe.

This is basically a declarative language for hanb.

## Palettes 
Not implemented

# Contributing

1. Fork the repo
2. Make your changes and update/add tests
3. Install pre-commit and install this repos hooks:
```sh
pip install pre-commit
cd /repo/root/path
pre-commit install
```
4. Commit your changes and pre-commit will auto format the source code.
    4.1. if you don't want to use pre-commit run this:
```sh
cd /repo/root/path
cd rust/
cargo fmt
cargo clippy
```
5. Open a PR
