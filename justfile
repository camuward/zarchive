default:
    @just --choose

ci: check-all test-all
    cargo clippy --all --all-features -- -D warnings
    cargo fmt --all -- --check

check:
    cargo clippy --all --all-features -- \
        -W clippy::all \
        -W clippy::pedantic

# powerset of features
check-all:
    cargo check --all --no-default-features
    cargo check --all --no-default-features --features "std"
    cargo check --all --no-default-features --features "zerocopy"
    cargo check --all --no-default-features --features "std,zerocopy"

test:
    cargo test --all --all-features

# powerset of features
test-all:
    cargo test --all --no-default-features
    cargo test --all --no-default-features --features "std"
    cargo test --all --no-default-features --features "zerocopy"
    cargo test --all --no-default-features --features "std,zerocopy"

run:
    cargo run

alias t := test
alias c := check
