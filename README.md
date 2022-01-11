# Othello-rs

## Playing Strength
Playing strength has been evaluated using an [Elo rating](https://en.wikipedia.org/wiki/Elo_rating_system) system
calibrated to 1 minute games on OthelloQuest.
Othello-rs is competitive with the best humans at a meager search depth of 2.

| Player                                           | Time per move (sec) | Elo  |
|--------------------------------------------------|---------------------|------|
| Random moves                                     | 0                   | 809  |
| Wold Champion Human                              | 2                   | 2450 |
| Othello-rs (book=false, middepth=6, enddepth=20) | 0.5                 | 3230 |
| Othello-rs (book=false, middepth=7, enddepth=20) | 1.5                 | 3320 |

## Opening Book

Othello-rs uses an opening book constructed similarly to [Saio](https://www.romanobenedetto.it/tesi.pdf).
The opening book has been computed for the first 25 ply and allows Othello-rs to (almost) guarantee at most a 2 disk
disadvantage after the first 25 ply.

## Midgame Search

The midgame search uses alpha-beta pruning in a fail-soft negamax framework.
It uses a residual neural network with 3.25M parameters as a static evaluation function.
The network was trained on 4.4 million Othello games played by strong computer engines.
The neural network is quite slow compared to traditional Othello evaluation functions, but it is much more accurate.
A single neural network evaluation is roughly equivalent to a 6 ply WZebra search.

## Endgame Solver

The endgame solver allows for solving positions with 20 empties in under 4 seconds.

## Building
Pytorch is used to perform inference with the neural network, so libtorch must be available on the system.

Build with
```shell
# build for CPU
fish build.fish
# build for GPU
fish build_gpu.fish
```
Alternatively, you can build with `cargo build --release`.
It is recommended that you build with `RUSTFLAGS="--emit=asm -C target-cpu=native -C opt-level=3"`.

## GPU Compute
The GPU can be used to accelerate the neural network evaluation.
GPU acceleration can be used by enabling the `gpu` feature or building with `build_gpu.fish`.

| Device      | Avg Move Time, depth=6 (sec) |
|-------------|------------------------------|
| R9 3950X    | 0.92                         |
| GTX 1080 Ti | 0.57                         |
| RTX 3090    | 0.41                         |


## Network Protocol
For an example, see the [Python client](clients/client.py) in the [clients](clients) directory.


```
The server expects 20 bytes in the format !QQHH (big-endian) of me, enemy, time, params.

me is a u64 for the bitboard of the current player

enemy is a u64 for the bitboard of the opponent

time is a u16 with the remaining time for the entire game in 1/10ths of a second

params is a u16 with layout __TBSADDDDDEEEEE
_ - bit(s) reserved for future use
T - bit to adjust based on time (1 = adjust params to fit in remaining time, 0 = ignore remaining time)
B - bit to use the opening book (1 = use book, 0 = no book)
S - bit to solve exact endgame (1 = exact, 0 = WLD)
A - bit to force WLD on deep endgame searches (WLD on eg depth > 15) (1 = WLD, 0 = exact)
D - 5 bits for neural network depth (0-31)
E - 5 bits for endgame depth (0-31)

The server will respond in the format !Bh (big-endian) of move, eval.

move is a u8 of the best move 0-63 or 64/65 for passing

eval is the evaluation of the board from the POV of the current player in centidisks
```

