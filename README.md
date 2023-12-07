# Rust advent 🦀 🐚

[![Rust](https://github.com/lmammino/rust-advent/actions/workflows/rust.yml/badge.svg)](https://github.com/lmammino/rust-advent/actions/workflows/rust.yml)

Learning Rust by implementing solutions for [Advent of Code](https://adventofcode.com/) problems.


🎥 **HEY, we are live-streaming our attempts to solve the exercises in this repo!** Check us out on:

  - [Twitch](https://twitch.tv/loige) for the live streams
  - [YouTube](https://www.youtube.com/channel/UCL0w2IAjTBx3NNka-l7InPw) for the recordings

[![Eugen, Roberto and Luciano trying to solve Advent of Code in Rust](https://i.imgur.com/wVIPDnt.png)](https://twitch.tv/loige)

And remember to follow and subscribe! 😎 😋

## Requirements

This repo contains some code that is bespoke to Raspberry Pi Pico, therefore you need to install some additional dependencies:

```bash
rustup target install thumbv6m-none-eabi
```

and

```bash
cargo install flip-link
```


## How to run tests for all exercises

Simply execute:

```bash
cargo test
```

If you want to run only one test for a given part of an exercise you can run something like this:

```bash
cargo test --package ex01 --lib --all-features -- tests::part_2
```


## Create a new exercise

**New**: You can now use the `scaffold.mjs` script to create a new exercise. Simply run:

```bash
./scaffold.mjs <year> <day>
```

(Note: it requires [`zx`](https://github.com/google/zx) to be installed).

---


Alternatively, you can create a new exercise manually by following these steps:


Cd into the specific **year folder** (e.g. `y2020`) and run:

```bash
cargo new --lib exNN
```

Replace `NN` with the number of exercise for the given year. For instance:


```bash
cargo new --lib ex01
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

Licensed under [MIT License](LICENSE). © Luciano Mammino, Roberto Gambuzzi, Eugen Serbanescu, Stefano Abalsamo.
