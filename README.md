# Rust advent 🦀 🐚


Learning Rust by implementing solutions for [Advent of Code](https://adventofcode.com/) problems.

## How to run an exercise

Simply execute:

```bash
cargo run y2020/ex01/
```


## Create a new exercise

Cd into the specific **year folder** (e.g. `y2020`) and run:

```bash
cargo new exNN
```

Replace `NN` with the number of exercise for the given year. For instance:


```bash
cargo new ex01
```

Finally add the new subproject in the workspace by editing the main [`Cargo.toml`](/Cargo.toml). For instance, assuming you just created `y2020/ex10`:


```toml
[workspace]
members = [
  "y2020/ex01",
  # ...
  "y2020/ex10" # <- new entry
]
```

## Contributing

Everyone is very welcome to contribute to this project.
You can contribute just by submitting bugs or suggesting improvements by
[opening an issue on GitHub](https://github.com/lmammino/rust-advent/issues).


## License

Licensed under [MIT License](LICENSE). © Luciano Mammino, Roberto Gambuzzi, Stefano Abalsamo.