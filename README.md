<div align="center">
    <h1>
        <img src="./static/icon.png" width="18" height="18" alt="" />
        Snake
    </h1>
    <a href="https://git.exst.fun/snake">
        <img src="https://img.shields.io/github/stars/eexsty/snake?colorA=1e1e28&colorB=1187c9&style=for-the-badge&logo=github" alt="GitHub" />
    </a>
    <a href="https://git.exst.fun/snake/actions/workflows/rust.yml">
        <img src="https://img.shields.io/github/workflow/status/eexsty/snake/Rust%20CI%20with%20Cargo?colorA=1e1e28&colorB=1187c9&label=Rust&style=for-the-badge&logo=rust" alt="GitHub Actions" />
    </a>
    <br/>
    <strong>A reimplementation of the classic snake game written in Rust, with desktop and web support via WebAssembly.</strong>
</div>


## Table of contents

1. [🚴 Prerequisites](#-prerequisites)
2. [🏠 Building](#-building)
3. [🔗 Attribution](#-attribution)


## 🚴 Prerequisites

* [Git][git]
* [Latest build of Rust Nightly][rustup]
* * `rustup target install wasm32-unknown-unknown`
* * `cargo install -f cargo-binutils` ![](./readme/windows-blue.svg)
* * `rustup component add llvm-tools-preview` ![](./readme/windows-blue.svg)
* [`rui314/mold` linker][mold] ![](./readme/ubuntu-blue.svg)
* [`michaeleisel/zld][zld] ![](./readme/apple-blue.svg)
* [Trunk][trunk]
* [NodeJS + NPM][nodejs]


## 🏠 Building

* `cargo build --release` ![](./readme/desktop-blue.svg)
* `trunk build --release` ![](./readme/web-blue.svg)


## 🔗 Attribution

1. [`tabler-icons.io`](https://tabler-icons.io) for their icons used in this markdown file.


[git]: https://git-scm.com/
[rustup]: https://rustup.rs
[trunk]: https://trunkrs.dev
[nodejs]: https://nodejs.org/en/
[mold]: https://github.com/rui314/mold
[zld]: https://github.com/michaeleisel/zld