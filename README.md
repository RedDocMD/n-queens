# N-Queens problem
This solves the N-Queen problem using **hill-climbing** with *random-restart* when a plateau is reached.

### Running 
To run the un-optimized build, execute `cargo run --bin nqueens <number>`, where `number` is the number of queens you want.

### Building 
To build the project, run `cargo build --release`. The built binary will be produced as `target/release/nqueens`. Run is as `target/release/nqueens <number>`, where `number` is the number of queens you want.

### Dependencies
This project is written in **Rust** ([Rust home page](https://www.rust-lang.org)).

You need to have `cargo` in your `PATH`. Check install instructions [here](https://www.rust-lang.org/tools/install).
