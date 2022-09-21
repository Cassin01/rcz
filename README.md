# rcz

A tool to write a commit message based on [Conventional Commits](https://www.conventionalcommits.org/)

## Installation

```sh
cargo install rcz
```

## Example

```zsh
function gitk {
    git commit -m "`rcz`"
}
```
