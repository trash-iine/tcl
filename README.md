# Trash-iine's programming competition libraries

# Generate snippets

## Preparation

You need to install `rustfmt` and `cargo-snippet`.

```bash
$ rustup component add rustfmt
$ cargo install cargo-snippet --features="binaries"
```

## Usage

Generate snippets for vscode.

```bash
$ mkdir .vscode
$ touch .vscode/rust.code-snippets
$ cargo snippet -t vscode > .vscode/rust.code-snippets
```

The snippets generated by the above commands are available only in this repository.