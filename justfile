build:
    cargo build --release

types:
    cargo test --package common
    cd common && pnpm run build
