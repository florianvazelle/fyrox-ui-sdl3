default:
    just --list

lint:
    cargo fmt --all --check
    cargo check --all-targets --examples --tests --all-features
    cargo clippy --all-targets --examples --tests --all-features -- --deny warnings
    typos
    taplo check *.toml
    find shaders/ -iname *.vert -o -iname *.frag -o -iname *.comp -o -iname *.glsl | xargs clang-format --dry-run

fmt:
    cargo fmt --all
    cargo fix --all-targets --examples --tests --allow-dirty --all-features
    cargo clippy --all-targets --fix --examples --tests --allow-dirty --all-features -- --deny warnings
    typos --write-changes
    taplo format *.toml
    find shaders/ -iname *.vert -o -iname *.frag -o -iname *.comp -o -iname *.glsl | xargs clang-format -i

build:
    cargo build --examples --all-targets --all-features