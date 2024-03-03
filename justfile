_default: help

# Prints available recipes
@help:
    echo "\nRun a task using \`just [RECIPE]\`."
    just --list

# --- Build & serve ---

# Clears build artifacts
clean:
    cargo clean

CORE_BUILD_PWA_ARGS := "--project core --bin-features pwa --lib-features hydrate,pwa,vite-prebuild"

_build-core-pwa *args:
    cargo leptos build --project core-pwa {{ args }}

# Builds the application in release mode
build:
    just _build-core-pwa --release
    cargo leptos build {{ CORE_BUILD_PWA_ARGS }} --release

# Runs development server without PWA features and watches for changes
dev:
    cargo leptos watch --project core

# Runs development server without PWA features
serve:
    cargo leptos serve --project core

# Runs development server including PWA features
serve-pwa:
    just _build-core-pwa
    cargo leptos serve {{ CORE_BUILD_PWA_ARGS }}

# Builds the application in release mode and serves it
serve-release:
    #!/usr/bin/env sh
    [ ! -f "target/server-release/core" ] && just build
    export LEPTOS_SITE_ADDR="127.0.0.1:3333"
    export LEPTOS_SITE_ROOT="target/client"
    export LEPTOS_SITE_PKG_DIR="hydrate"
    export LEPTOS_HASH_FILES=true
    ./target/server-release/core

SKIP_WASM_CHECKS_PACKAGES := trim(replace_regex('''
    core
    server
''', '\s+', ' '))
skip_wasm_checks_workspace_exclude := '--exclude ' + replace_regex(SKIP_WASM_CHECKS_PACKAGES, ' ', ' --exclude ')

# --- Test ---

_test-native *args:
    cargo nextest run --workspace --cargo-profile server-dev {{ args }}

# Runs tests for server target
test-native *filter:
    just _test-native -- {{ filter }}

# Runs tests for wasm target
test-wasm *args:
    cargo test --profile client-dev \
        --target wasm32-unknown-unknown \
        --workspace {{ skip_wasm_checks_workspace_exclude }} \
        {{ args }}

# Runs tests
test *filter:
    # cargo-nextest doesnt't support doctests yet: https://github.com/nextest-rs/nextest/issues/16
    cargo test {{ filter }} --doc --workspace --profile server-dev
    just test-native {{ filter }}
    just test-wasm {{ filter }}

# Compile tests without running them
build-tests *args:
    just _test-native --no-run {{ args }}
    just test-wasm --no-run {{ args }}

# Reviews test snapshots generated by `insta`
review-snaps:
    cargo insta review

# Generate E2E selectors from `pin_test_selector!` macro invocations
generate-e2e-selectors:
    cargo check \
        -p client \
        --target wasm32-unknown-unknown \
        --profile client-dev \
        --features csr,pwa,e2e-selectors

# Serves the app in release mode and runs E2E tests with Playwright
e2e *args:
    just generate-e2e-selectors
    npx playwright test --config e2e/playwright.config.ts {{ args }}

# Serves the app in release mode and opens Playwright UI
e2e-ui:
    just e2e --ui

# --- Formatting ---

# Formats Rust files using rustfmt
_fmt-rustfmt *args:
    rustfmt apps/**/*.rs packages/**/*.rs {{ args }}

# Formats Leptos components using leptosfmt
_fmt-leptosfmt *args:
    leptosfmt packages/**/components/**/*.rs {{ args }}

# Formats Rust files including Leptos component syntax
fmt-rs:
    just _fmt-rustfmt
    just _fmt-leptosfmt

_prettier +args:
    npx prettier "**/*.{html,yaml,yml,toml}" "!pnpm-lock.yaml" {{ args }}

# Formats supported files with Biome
_fmt-biome:
    npx biome check . --apply

# Formats files not supported by Biome with Prettier
_fmt-prettier: (_prettier "-w")

# Formats justfile
_fmt-justfile:
    just --unstable --fmt

# Formats all non-Rust files using Biome and Prettier
fmt-non-rs:
    #!/usr/bin/env sh
    (
        just _fmt-biome &
        just _fmt-prettier &
        just _fmt-justfile &
        wait
    )

# Formats all files in the project
fmt:
    #!/usr/bin/env sh
    (
        just fmt-rs &
        just fmt-non-rs &
        wait
    )

_fmt-check-rustfmt:
    just _fmt-rustfmt --check

_fmt-check-leptosfmt:
    just _fmt-leptosfmt --check

_fmt-check-biome:
    npx biome check .

_fmt-check-prettier:
    just _prettier --check

_fmt-check-justfile:
    just --unstable --fmt --check

# Checks formatting
fmt-check:
    just _fmt-check-rustfmt
    just _fmt-check-leptosfmt
    just _fmt-check-biome
    just _fmt-check-prettier
    just _fmt-check-justfile

# --- Lint ---

_check message_format *args='--profile server-dev':
    cargo check --all-targets --workspace --message-format={{ message_format }} {{ args }}

_clippy message_format *args='--profile server-dev':
    cargo clippy --all-targets --workspace --message-format={{ message_format }} {{ args }}

_check-wasm message_format:
    just _check {{ message_format }} \
        --profile client-dev \
        --target wasm32-unknown-unknown \
        {{ skip_wasm_checks_workspace_exclude }}

# `clippy::str-to-string` is triggered by Leptos' `#[server]` macro for the wasm triple, therefore it's disabled here
_clippy-wasm message_format *lint_args:
    just _clippy {{ message_format }} \
        --profile client-dev \
        --target wasm32-unknown-unknown \
        {{ skip_wasm_checks_workspace_exclude }} \
        -- -A clippy::str-to-string {{ lint_args }}

# Checks Rust codebase
check:
    just _check human
    just _check-wasm human

_lint-rs *lint_args:
    just _clippy human --profile server-dev -- {{ lint_args }}
    just _clippy-wasm human {{ lint_args }}

# Lints Rust codebase with Clippy
lint-rs: _lint-rs

# Script made for IDEs to support server and wasm targets at the same time. `cargo check` is fired first because it's much faster than `cargo clippy`.
_rust-analyzer-check:
    #!/usr/bin/env sh
    (
        (
            just _check json;
            just _clippy json
        ) &
        (
            just _check-wasm json;
            just _clippy-wasm json
        ) &
        wait
    )

# Checks for TypeScript errors
lint-ts:
    just generate-e2e-selectors
    npx tsc -p packages/client
    npx tsc -p e2e

# Checks for typos
lint-typos:
    typos

# Lints the project
lint: lint-rs lint-ts lint-typos

# Lints the project without optimizations and disallows warnings
lint-ci:
    just _lint-rs -D warnings
    just lint-ts lint-typos

# --- Security ---

# Runs Cargo dependency audit
audit-rs:
    cargo audit

# Runs Node dependency audit
audit-js:
    pnpm audit --prod

# Runs all dependency audits
audit: audit-rs audit-js

# --- Dependency management ---

CARGO_EXECUTABLES := replace_regex('''
just@1.24.0
cargo-leptos@0.2.12
leptosfmt@0.1.18
cargo-nextest@0.9.67
cargo-outdated@0.14.0
cargo-audit@0.20.0
cargo-udeps@0.1.46
typos-cli@1.18.2
wasm-bindgen-cli@0.2.91
''', '\s+', ' ')
CARGO_DEV_EXECUTABLES := replace_regex('''
cargo-expand@1.0.79
cargo-edit@0.12.2
cargo-insta@1.35.1
''', '\s+', ' ')

_setup +executables:
    #!/usr/bin/env sh
    (
        # Create local .cargo/config.toml file
        [ ! -f .cargo/config.toml ] && cp .cargo/config.example.toml .cargo/config.toml

        # Rust toolchain
        rustup toolchain install nightly \
            --profile minimal \
            -c rustfmt clippy rustc-codegen-cranelift-preview
        rustup target add wasm32-unknown-unknown

        # cargo-binstall
        curl -L --proto '=https' --tlsv1.2 \
            -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh \
            | bash > /dev/null

        # Node dependencies
        # (should not run in parallel with other stuff,
        # because `playwright install --with-deps` may ask for permissions)
        npm i -g pnpm
        pnpm i --frozen-lockfile
        npx playwright install --with-deps

        # Cargo executables
        for dep in {{ executables }}; do
            cargo binstall $dep -y --only-signed --no-discover-github-token --log-level error &
        done

        wait
    )

# Performs complete project setup and clears previous build artifacts
setup:
    just _setup {{ CARGO_EXECUTABLES }} {{ CARGO_DEV_EXECUTABLES }}
    just clean

# Performs project setup, but skips dependencies unused in CI
setup-ci:
    just _setup {{ CARGO_EXECUTABLES }}

# Updates Cargo packages to the latest versions in their specified ranges
update-rs:
    cargo upgrade --recursive
    cargo update

# Updates Node packages to the latest versions in their specified ranges
update-js:
    pnpm update --recursive

# Updates packages to the latest versions in their specified ranges
update: update-rs update-js

# Updates Cargo packages to their latest versions ignoring ranges in Cargo.toml
update-rs-latest:
    cargo upgrade --recursive --incompatible allow
    cargo update

# Updates Node packages to their latest versions ignoring ranges in package.json
update-js-latest:
    pnpm update --latest --recursive

# Updates packages to the latest versions ignoring their ranges
update-latest: update-rs-latest update-js-latest

# Checks for outdated Cargo dependencies
outdated-rs:
    cargo outdated --workspace --root-deps-only --exit-code 1

# Checks for outdated Node dependencies
outdated-js:
    pnpm outdated --recursive

# Checks for outdated dependencies
outdated: outdated-rs outdated-js

# Checks for unused dependencies
unused:
    cargo udeps --workspace --all-targets
    cargo udeps --workspace --all-targets \
        --target wasm32-unknown-unknown \
        {{ skip_wasm_checks_workspace_exclude }}

# --- Dev tools ---

# Expands macros
expand package *args:
    cargo expand -p {{ package }} --lib --profile server-dev {{ args }}

# Expands macros for wasm target
expand-wasm package *args:
    cargo expand -p {{ package }} --lib --profile client-dev --target wasm32-unknown-unknown {{ args }}

# FIXME: passing profiles to `cargo expand` makes it unable to find test modules
# and the recipe below is a workaround for that.

# Expands macros for test modules
expand-test package *args:
    cargo expand -p {{ package }} --lib --tests {{ args }}
