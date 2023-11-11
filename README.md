<p align="center">
  <img src="https://api.iconify.design/simple-icons/leptos.svg?color=%23d74f3f" width="200"/>
</p>

<h2 align="center">
  Leptos Assembly
</h2>

<p>
  <i>
    <div align="center">
      Leptos / Axum / PWA / I18n
    </div>
    <div align="center">
      UnoCSS / Iconify / cargo-leptos & Vite / Playwright
    </div>
  </i>
</p>

<p align="center">
  Full stack, fully packed
</p>

<p align="center">
  ☁️ <a href="#">//TODO: add online demo</a> ☁️
</p>

> This project is in a heavy work-in-progress state with lots of hacks to put it all together, vague design choices and some missing features. Plenty of things are yet to be cleaned up or extracted into separate crates.\
> You shouldn't really use it at all. Feel free to salvage some parts of it could you find them useful though.

## 💡 About this project

This project is a template for a full-stack web application written in Leptos. It is meant to be used as a starting point for new projects with a complete collection of crates and other tools that work well together.\
It is inspired by [Vitesse](https://github.com/antfu/vitesse) and its role during early days of Vue 3, when Nuxt 3 was not yet available. The goal is to provide ready-to-use box of tools rather than a raw template.\
Additionally, the project aspires to implement a strict, predictable and scalable architecture that is easy to follow and build upon. You will find [guidelines](#-file-architecture) below and there's a plenty of redundant code here that can help you getting familiar with the rules.

## 📦 Features

### Core frameworks:

- [Leptos](https://leptos.dev/)

- [Axum](https://github.com/tokio-rs/axum)

### Core plugins:

- [Routing](https://leptos-rs.github.io/leptos/router/index.html) - necessity for most apps nowdays

- [Leptos-Use](https://leptos-use.rs/) - lots of useful stuff

- [Leptos i18n](https://github.com/Baptistemontan/leptos_i18n) - internationalization

### UI tools:

- [UnoCSS](https://github.com/unocss/unocss) - TailwindCSS, but better

- [Iconify](https://icones.js.org/) - works well with UnoCSS

### Coding style:

- pedantic [clippy](https://github.com/rust-lang/rust-clippy) lints

- [rustfmt](https://github.com/rust-lang/rustfmt) + [leptosfmt](https://github.com/bram209/leptosfmt) formatters

- formatting / linting non-Rust codebase is done with [Biome](https://biomejs.dev/) and [Prettier](https://prettier.io/) (cuz Biome doesn't support everything in this project yet)

- [TypeScript](https://www.typescriptlang.org/) is used where some JS code is needed (e.g. bindings with Node dependencies, E2E tests)

### Other notable toys:

- [just](https://github.com/casey/just) - command runner, just as you expected

- [cargo-leptos](https://github.com/leptos-rs/cargo-leptos) - the main build tool and development server

- [Vite](https://vitejs.dev/) - secondary bundler used for prebuilding the application. It is meant to provide a bridge between rusty WASM world and JS plugins.

- [vite-plugin-pwa](https://github.com/vite-pwa/vite-plugin-pwa) - you probably don't want your users to download that fat `.wasm` file every time they visit the website

- [Webfont Dowload Vite plugin](https://github.com/feat-agency/vite-plugin-webfont-dl) - self-hoisted Google fonts without polluting our repository with them

- [pnpm](https://pnpm.io/) - package manager for Node dependencies

- [Playwright](https://playwright.dev/) - End-to-End testing

- [cargo-expand](https://crates.io/crates/cargo-expand) - macro expansion

- [cargo-edit](https://crates.io/crates/cargo-edit) - dependency management

- [cargo-audit](https://crates.io/crates/cargo-audit) - checking for security vulnerabilities

- [cargo-outdated](https://crates.io/crates/cargo-outdated) - detecting outdated dependencies

## 🛠️ Project setup

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) obviously.

- some CLI tools may require additional OS packages:

  - Ubuntu:

    ```bash
    sudo apt install -y build-essential libssl-dev
    ```

- [just](https://github.com/casey/just) is required for running CLI tasks.\
  Installation:

  ```bash
  cargo install just
  ```

- [Node](https://nodejs.org/en) - the exact version is specified in `.nvmrc` file.\
  Installation using [nvm](https://github.com/nvm-sh/nvm):

  ```bash
  nvm install
  nvm use
  ```

Once you have everything, you can simply run:

```bash
just setup
```

This will install all the Cargo executables, crates and Node dependencies necessary to develop and build the project. **You're ready to go!**

In order to improve compilation times, you can also setup [mold](https://github.com/rui314/mold) locally. `.cargo/config.toml` is skipped from version control, so you append your very own adjustments:

```toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=/path/to/mold"]

```

You may also need `clang` to use `mold`:

- Ubuntu:

  ```bash
  sudo apt install clang
  ```

### 🔥 Compile and hot-reload for development

```bash
just dev
```

### 🧹 Format the codebase

```bash
just fmt
```

### 🩺 Lint

```bash
just lint
```

### 🧪 Run tests

```bash
just test
```

### 🎭 Run E2E tests with [Playwright](https://playwright.dev/)

```bash
just e2e
just e2e-ui
```

### 🚀 Compile and minify for production

```bash
just build
```

## 🌱 File architecture

// TODO: make sure its good enough and add exhaustive guide

```bash
📁project
├─ 📁apps
│  └─ 📁core
│     ├─ 🚩main.rs
│     └─ ⚙️Cargo.toml
│
├─ 📁e2e
├─ 📁packages
│  ├─ 📁common
│  │  ├─ 📁macros
│  │  │  ├─ 📁proc
│  │  │  │  ├─ 🚩lib.rs
│  │  │  │  └─ ⚙️Cargo.toml
│  │  │  ├─ 🚩lib.rs
│  │  │  └─ ⚙️Cargo.toml
│  │  └─ 📁utils
│  │     ├─ 🚩lib.rs
│  │     └─ ⚙️Cargo.toml
│  │
│  ├─ 📁client
│  │  ├─ 📁components
│  │  │  ├─ 🚩lib.rs
│  │  │  └─ ⚙️Cargo.toml
│  │  ├─ 📁composables
│  │  │  ├─ 🚩lib.rs
│  │  │  └─ ⚙️Cargo.toml
│  │  ├─ 📁macros
│  │  │  ├─ 📁proc
│  │  │  │  ├─ 🚩lib.rs
│  │  │  │  └─ ⚙️Cargo.toml
│  │  │  ├─ 🚩lib.rs
│  │  │  └─ ⚙️Cargo.toml
│  │  ├─ 📁pages
│  │  │  └─ 📁home
│  │  │     ├─ 📁components
│  │  │     ├─ 📁composables
│  │  │     ├─ 🚩lib.rs
│  │  │     └─ ⚙️Cargo.toml
│  │  ├─ 📁router
│  │  ├─ 📁styles
│  │  ├─ 📁utils
│  │  │  ├─ 🚩lib.rs
│  │  │  └─ ⚙️Cargo.toml
│  │  ├─ 🚩lib.rs
│  │  └─ ⚙️Cargo.toml
│  └─ 📁server
│     ├─ 🚩lib.rs
│     └─ ⚙️Cargo.toml

```

## 🏁 Checklist

- [ ] Change the project name in `Cargo.toml` and `package.json`
- [ ] Change the author name in `LICENSE` or simply remove the file
- [ ] Change the favicon in `apps/client/public/`

## 🏷️ License

[MIT License](https://opensource.org/licenses/MIT)

Copyright (c) 2023-PRESENT Kajetan Welc

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
