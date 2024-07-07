# Conway's Game of Life

## Description

This is a [Rust](https://www.rust-lang.org) implementation of the Game of Life, a cellular automaton devised by John Horton Conway.

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run
```

## Dependencies

```toml
[dependencies]
crossterm = "0.27.0"
rand = "0.8.5"
termsize = "0.1.8"
ctrlc = "3.4.4"
```

## License

MIT License