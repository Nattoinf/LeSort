default:
    just --list

build:
    cargo build

release:
    cargo build --release

run:
    cargo run -- .

test:
    cargo test

fmt:
    cargo fmt

clippy:
    cargo clippy -- -D warnings

coverage:
    cargo llvm-cov --html

doc:
    cargo doc --open

docker-build:
    docker build -t lesort .

docker-run:
    docker run --rm -v "$(pwd)":/work lesort /work -s

clean:
    cargo clean
