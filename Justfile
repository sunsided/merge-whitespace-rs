[private]
help:
    @just --list --unsorted

# Builds the project using cargo
build:
    cargo build

# Runs the tests
test:
    cargo test --lib --no-default-features
    cargo test --lib --all-features
    cargo test --doc --no-default-features
    cargo test --doc --all-features
    cargo doc

# Builds and opens the documentation
doc:
    cargo doc --open

# Runs fuzzing for a provided target
fuzz TARGET:
    cargo +nightly fuzz run "{{ TARGET }}" -- -max_total_time=60

# Runs code coverage
codecov PROFILE="ci":
    cargo llvm-cov nextest --all-features --lcov --output-path lcov.info "--profile={{ PROFILE }}"
