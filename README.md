# wordle-rs

A command-line clone of the popular [Wordle](https://www.powerlanguage.co.uk/wordle/) game written in rust.

It uses the same word list as the official game, so you can compete against other players. Alternatively, you can pass the `-r` command line option, which will pick a random 5-letter word. 

## Running
Make sure you have rust and cargo installed, and then run `cargo run` or `cargo run -- -r` in your terminal.

## Building
Make sure you have rust and cargo installed, and then run `cargo build` in your terminal for an unoptimized/dev build, or `cargo build --release` for an optimized build.

