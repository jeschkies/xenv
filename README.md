# Xenv the environment version manager

Xenv is very similar to [pyenv](https://github.com/pyenv/pyenv) and [rbenv](https://github.com/rbenv/rbenv) except that
it manages the version of multiple tools instead of one.

## Install

Simply install `xenv` with [Cargo](https://doc.rust-lang.org/cargo/) by running `cargo install xenv`. You can also build
`xenv` with `cargo build --release`. Once installed add `eval "$(xenv hook zsh)"` to your `~/.zshrc`.

## Usage

`xenv` looks for a local `.xenv.toml` in your folder to set the `PATH` and `LIBRARY` environnment variables according to
the desired versions. To get started create a `.xenv.toml` with

```toml
[packages]
mesos=1.7.2
```
