# N-Queens problem

This solves the N-Queen problem using **hill-climbing** with *random-restart* when a plateau is reached.

The following variants of the above algorithm have been implemented (they differ in the manner the data is represented as well as the method used to find the subsequent neighbor):

1. Square board, tiles move vertically
2. Linearized board, tiles move vertically
3. QS1 algorithm

## Running

### Building

To build the project, run `cargo build --release`. The binaries will be produced under `target/release`.

### Options

The binaries take in the number of queens as a command-line arg. So run the binaries as `target/release/<binary> <number-of-queens>`.

### Demo run
Also you can get a brief idea of the comparative performances of the three algorithms by using `test_run.py`, if you have Python3 and *termcolor* installed.
```
pip3 install termcolor
python3 test_run.py
```

Alternatively, if you have [Docker](https://www.docker.com/) installed, you can run the above script as a container, with:
```
docker run --rm redocmd/n-queens
```

### Dependencies

This project is written in **Rust** ([Rust home page](https://www.rust-lang.org)).

You need to have `cargo` in your `PATH`. Check install instructions [here](https://www.rust-lang.org/tools/install).

## Algorithms

### Square board, tiles move vertically

In this variant, a 2D vector of booleans is used to represent the board, with a `true` representing a queen at that position. The initial board is created by placing the queens in a random row, one per column. (Note that this introduces row-conflicts in addition to diagonal conflicts. Without these initial row-conflicts, however, it was observed that many more random restarts were required).

At each move, `n(n -1)` possible successors are explored. A successor is obtained by shifting a queen vertically. The best possible successor is chosen. If it has a better value than the current state, then it becomes the current state. Else the search restarts from a new random position.

#### Heuristic

The heuristic used here is the *total* number of pairs queens conflicting each other, either *directly or indirectly*. So for an instance, if there are 4 queens in a line, that contributes 6 (`C(4, 2)`) to the heuristic value. Also, note that the goal state has a heuristic value of 0, making this algorithm more properly *valley-descending*, rather than hill-climbing.

The binary for this variant is `full_board`.

### Linearized board, tiles move vertically

In this, variant the board is represented as a vector of positions, where the value is the ith element represents the row the queen in the ith column is in. As a consequence, the heuristic is calculated based on the previous value of the heuristic, in O(n) time, as compared to O(n^2) time in the previous variant. Also the goal test is reduced to O(1) time from the O(n^2) from the previous variant.

#### Heuristic

Same as the previous variant.

The binary for this variant is `linear_board`.

### QS1 algorithm

This has been taken from Sosic and Gu ([https://doi.org/10.1145/101340.101343](https://doi.org/10.1145/101340.101343)). The board is represented as the same linearized format as above. The initial random permutation, however, is guaranteed to have neither row conflicts nor column conflicts. The local search proceeds by comparing all pairs of queens. If one of them is currently attacked, then those two queens are swapped, iff the swap will improve the heuristic. The process of recalculating the heuristic in O(1) time has been described in the paper.

#### Heuristic

In this variant, the heuristic is the *total* number of pairs of queens in **direct** conflict.

The binary for this variant is `qs1`.
