# `uni`

Comically short program to print out the Greek alphabet to `stdout`.

This was designed for the [helix](https://helix-editor.com/), a terminal based text editor.
Although it does not have snippets, one can easily pipe shell outputs into the editor with `!`, hence the motivation for this program.

In this way, if I need the Greek letter $\beta$, I can just run `!uni beta` and it will appear at the current cursor position.

## Installation

Assuming [Rust](https://www.rust-lang.org/) is installed, it is just a matter of

```
git clone https://github.com/ElisR/uni.git
cd uni
cargo install --path .
```
