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
| Neovim | still Neovim, but with [bob](https://github.com/MordechaiHadad/bob), or [helix](https://helix-editor.com/), or [zed](https://zed.dev) |
| curl | [hurl](https://hurl.dev) |
| nvm/rubyenv/pyenv | [mise](https://mise.jdx.dev/) |
| cd/z | [zoxide](https://github.com/ajeetdsouza/zoxide) |

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

- Rust is a very slow to write in language
    - But a very correct language!
- Steep learning curve
- Integrating with existing JS libraries is a bit of a pain
- Async traits have caveats (for library developers mostly)

## Others

[Energy efficiency across programming languages](https://greenlab.di.uminho.pt/wp-content/uploads/2017/10/sleFinal.pdf)
[Cargo script?](https://github.com/rust-lang/rfcs/pull/3503#issuecomment-1930765966)
[LINQ in Rust](https://github.com/StardustDL/Linq-in-Rust)
[Python-like list comprehension](https://github.com/mattgathu/cute)
[Are we learning yet?](https://www.arewelearningyet.com/gpu-computing/)
[Are we web yet?](https://www.arewewebyet.org/)

[mk48.io](https://mk48.io)
