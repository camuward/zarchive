host_target := `rustc --version --verbose | grep host | cut -d' ' -f2`
tools := "cargo cargo-objdump grep just jq perl rustfilt"

[private]
default: requirements
    @just --choose

[private]
powerset *list:
    #!/usr/bin/env perl
    my @input = split /\s+/, '{{ list }}';
    my @output = ('');

    for my $item (@input) {
        my @children;
        for my $prev (@output) {
            my $child = ($prev ne '') ? "$prev,$item" : $item;
            push @children, $child;
        }
        push @output, @children;
    }

    print join("\n", @output);
    print "\n";

[private]
foreach +command:
    #!/usr/bin/env bash
    set -euo pipefail

    features=`cargo read-manifest --quiet | jq -r '.features | keys[]' | grep -v default | tr '\n' ' '`
    feature_powerset=`just powerset $features`

    echo "==> using features:"
    {{ command }} ""

    for combo in $feature_powerset; do
        echo "==> using features: $combo"
        {{ command }} "$combo"
    done

requirements:
    #!/usr/bin/env bash
    set -euo pipefail

    for tool in {{ tools }}; do
        if ! command -v $tool &> /dev/null; then
            echo "error: $tool is required"
            exit 1
        fi
    done

# check all combinations of features
check-all:
    just foreach cargo check --all --no-default-features --features

# run all tests with all combinations of features
test-all:
    just foreach cargo test --all --no-default-features --features

alias c := check-all
alias t := test-all
alias r := run

doc:
    cargo +nightly doc --all-features \
        --document-private-items \
        -Zunstable-options \
        -Zrustdoc-scrape-examples
    $BROWSER target/doc/zarchive2/index.html

# pedantic clippy
clippy:
    cargo clippy --all --all-features -- \
        -W clippy::all \
        -W clippy::pedantic

# run all tests
test:
    cargo test --all --all-features

# run the default binary or specify an example
run *example:
    @if [ -z "{{ example }}" ]; then \
        cargo run --all-features; \
    else \
        cargo run --example "{{ example }}" --all-features; \
    fi

ci: check-all test-all
    cargo clippy --all --all-features -- -D warnings
    cargo fmt --all -- --check

disasm target=host_target:
    cargo-objdump
#    RUSTFLAGS="--crate-type=cdylib"
#    cargo rustc --release --target {{ target }} \
#        | llvm-objdump \
#            -x86-asm-syntax=intel \
#            -disassemble target/release/libzarchive2.a \
#        | rustfilt \
#            > src/main.S
