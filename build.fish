env RUSTFLAGS="--emit=asm -C target-cpu=native -C opt-level=3" cargo build --release --features bmi2
