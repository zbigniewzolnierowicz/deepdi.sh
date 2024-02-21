# Recipes

## Tools used:

### In the app proper

| Usual | New |
|:------| --- |
| postgres | [SurrealDB](https://surrealdb.com/) (not using this, I wanna show off sqlx) |
| Spring | [Actix Web](https://actix.rs/) |
| React | [Leptos](https://leptos.dev/) |
| Minio | [Garage](https://garagehq.deuxfleurs.fr/) |
| curl | [hurl](https://hurl.dev) |

### In my terminal

| Usual | New |
|:------| --- |
| tmux | [zellij](https://zellij.dev) |
| Neovim | still Neovim, but with [bob](https://github.com/MordechaiHadad/bob), or [helix](https://helix-editor.com/), or [zed](https://zed.dev) |
| nvm/rubyenv/pyenv | [mise](https://mise.jdx.dev/) |
| cd/z | [zoxide](https://github.com/ajeetdsouza/zoxide) |
| oh-my-zsh prompt | [starship](https://starship.rs) |
| grep | [ripgrep](https://github.com/BurntSushi/ripgrep) |
| find | [fd](https://github.com/sharkdp/fd) |
| cat | [bat](https://github.com/sharkdp/bat) |
| du | [dust](https://github.com/bootandy/dust) |
| ls | [eza](https://eza.rocks) |
| lazygit | [gitui](https://github.com/extrawurst/gitui) |
| vscode | [neovide](https://neovide.dev) |
| tldr | [tealdeer](https://github.com/dbrgn/tealdeer) |
| zsh | [nu](https://nushell.sh) (but I'm not using this one, too used to zsh) |
| ctrl-r | [mcfly](https://github.com/cantino/mcfly) && [mcfly-fzf](https://github.com/bnprks/mcfly-fzf) |

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
- Cannot use pooled connections with Neon ([issue in sqlx](https://github.com/launchbadge/sqlx/issues/67), just use normal connections rather than Neon's connection pooling)

## Others

[Energy efficiency across programming languages](https://greenlab.di.uminho.pt/wp-content/uploads/2017/10/sleFinal.pdf)
[Cargo script?](https://github.com/rust-lang/rfcs/pull/3503#issuecomment-1930765966)
[LINQ in Rust](https://github.com/StardustDL/Linq-in-Rust)
[Python-like list comprehension](https://github.com/mattgathu/cute)
[Are we learning yet?](https://www.arewelearningyet.com/gpu-computing/)
[Are we web yet?](https://www.arewewebyet.org/)
[sccache](https://github.com/mozilla/sccache)

[mk48.io](https://mk48.io)
