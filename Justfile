benchmark: build_rust build_c
    hyperfine './target/release/nbody 50000000' './nbody_c 50000000'

clean:
    cargo clean
    rm -f nbody_c


build_rust:
    cargo build --release

run_rust: build_rust
    ./target/release/nbody 50000000


build_c:
    gcc -pipe -Wall -O3 -fomit-frame-pointer -funroll-loops -march=native -static nbody.c -o nbody_c -lm

run_c: build_c
    ./nbody_c 50000000
