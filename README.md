# Recipes

## Tools used:

| Usual | New |
|:------| --- |
| git   | [~~jj~~](https://github.com/martinvonz/jj) (I ended up not using this) |
| postgres | [SurrealDB](https://surrealdb.com/) (not using this, I wanna show off sqlx) |
| Spring | [Actix Web](https://actix.rs/) |
| React | [Leptos](https://leptos.dev/) |
| Minio | [Garage](https://garagehq.deuxfleurs.fr/) |
| tmux | [zellij](https://zellij.dev) |
| Neovim | still Neovim, but with [bob](https://github.com/MordechaiHadad/bob), or [helix](https://helix-editor.com/) |
| curl | hurl |

## Prerequisites

```sh
# Installing tools we need

cargo install cargo-binstall
cargo binstall trunk
cargo binstall hurl

# Setting compilation targets && switching to nightly compiler (needed for Leptos)

rustup toolchain add nightly
rustup override set nightly
cd frontend
rustup target add wasm32-unknown-unknown
rustup component add rust-analyzer
```

## Issues

- Rust is a very slow language
    - But a very correct language!
- Integrating with existing JS libraries is a bit of a pain

## Others

[https://github.com/StardustDL/Linq-in-Rust](https://github.com/StardustDL/Linq-in-Rust)
[https://github.com/mattgathu/cute](https://github.com/mattgathu/cute)

[mk48.io](https://mk48.io)
