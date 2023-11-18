_default: help

# Prints avialable recipes
@help:
    echo "\nRun a task using \`just [RECIPE]\`."
    just --list

# --- Build & serve ---

# Clears build artifacts
clean:
    cargo clean

PWA_BUILD_ARGS := "--bin-features pwa --lib-features csr,pwa"

# Builds the application in release mode
build:
    cargo leptos build {{ PWA_BUILD_ARGS }} --release

# Runs development server without PWA features and watches for changes
dev:
    cargo watch -x 'leptos serve'

# Runs development server without PWA features
serve:
    cargo leptos serve --project core

# Runs development server including PWA features
serve-pwa:
    cargo leptos serve {{ PWA_BUILD_ARGS }}

# Serves the application in release mode
serve-release:
    cargo leptos serve {{ PWA_BUILD_ARGS }} --release

# --- Test ---

# Runs tests
test:
    # cargo-nextest doesnt't support doctests yet: https://github.com/nextest-rs/nextest/issues/16
    cargo test --doc
    # TODO add some example dummy tests
    cargo nextest run

# Runs tests without optimizations
test-ci:
    cargo test --doc --profile dev-ci
    cargo nextest run --cargo-profile dev-ci

_e2e:
    npx playwright test

_e2e-ui:
    npx playwright test --ui

# Serves the app and runs E2E tests with Playwright
e2e:
    just _e2e

# Serves the app in release mode and runs E2E tests with Playwright
e2e-release:
    PW_WEBSERVER_RELEASE_MODE=true just _e2e

# Serves the app and opens Playwright UI
e2e-ui:
    just _e2e-ui

# Serves the app in release mode and opens Playwright UI
e2e-ui-release:
    PW_WEBSERVER_RELEASE_MODE=true just _e2e-ui

# --- Formatting ---

# Formats Rust files using rustfmt
_fmt-rustfmt flag='':
    #!/usr/bin/env sh
    (
        for f in `find apps -name '*.rs'`; do
            rustfmt $f {{ flag }} &
        done
        for f in `find packages -name '*.rs'`; do
            rustfmt $f {{ flag }} &
        done
        wait
    )

# Formats Leptos components using leptosfmt
_fmt-leptosfmt flag='':
    leptosfmt packages/**/components/**/*.rs {{ flag }}

# Formats Rust files including Leptos component syntax
fmt-rs:
    just _fmt-rustfmt
    just _fmt-leptosfmt

_prettier flag:
    npx prettier "**/*.{html,yaml,yml,toml}" "!pnpm-lock.yaml" {{ flag }}

# Formats supported files with Biome
_fmt-biome:
    npx biome format . --write

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

# Checks Rust codebase
check:
    cargo check --workspace

# Lints Rust codebase with Clippy
lint-rs:
    cargo clippy

# Checks for TypeScript errors
lint-ts:
    npx tsc

# Lints the project
lint: lint-rs lint-ts

# Lints the project without optimizations and disallows warnings
lint-ci:
    cargo clippy --profile dev-ci -- -D warnings
    just lint-ts

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
just@1.15.0
cargo-leptos@0.2.2
leptosfmt@0.1.17
cargo-nextest@0.9.61
cargo-outdated@0.13.1
cargo-audit@0.18.2
cargo-udeps@0.1.43
''', '\s+', ' ')
CARGO_DEV_EXECUTABLES := replace_regex('''
cargo-expand@1.0.74
cargo-edit@0.12.2
cargo-watch@8.4.1
''', '\s+', ' ')

_setup +executables:
    #!/usr/bin/env sh
    (
        # Create local .cargo/config.toml file
        [ ! -f .cargo/config.toml ] && cp .cargo/config.example.toml .cargo/config.toml

        # Rust toolchain
        rustup toolchain install nightly --profile minimal -c rustfmt clippy rustc-codegen-cranelift-preview
        rustup target add wasm32-unknown-unknown

        # cargo-binstall
        curl -L --proto '=https' --tlsv1.2 \
            -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh \
            | bash > /dev/null

        # Node dependencies
        # (should not run in parallel with other stuff,
        # because `playwright install-deps` can ask for permissions)
        npm i -g pnpm
        pnpm i --frozen-lockfile
        npx playwright install --with-deps

        # Cargo executables
        for dep in {{ executables }}; do
            cargo binstall -y --only-signed --no-discover-github-token $dep &
        done

        wait
    )

# Performs complete project setup and clears previous build artifacts
setup:
    just _setup {{ CARGO_EXECUTABLES }} {{ CARGO_DEV_EXECUTABLES }}
    just clean

# Performs project setup, but skips dependencies ununsed in CI
setup-ci:
    just _setup {{ CARGO_EXECUTABLES }}

# Updates Cargo packages to the latest versions in their specified ranges
update-rs:
    cargo upgrade

# Updates Node packages to the latest versions in their specified ranges
update-js:
    pnpm update

# Updates packages to the latest versions in their specified ranges
update: update-rs update-js

# Updates Cargo packages to their latest versions ignoring ranges in Cargo.toml
update-rs-latest:
    cargo upgrade --incompatible allow

# Updates Node packages to their latest versions ignoring ranges in package.json
update-js-latest:
    pnpm update --latest

# Updates packages to the latest versions ignoring their ranges
update-latest: update-rs-latest update-js-latest

# Checks for outdated Cargo dependencies
outdated-rs:
    cargo outdated --root-deps-only --exit-code 1

# Checks for outdated Node dependencies
outdated-js:
    pnpm outdated

# Checks for outdated dependencies
outdated: outdated-rs outdated-js

# Checks for unused dependencies
unused:
    cargo udeps --workspace

# --- Misc ---

_vscode-fmt:
    # Using `leptosfmt --stdin --rustfmt` seems to add redundant newlines
    leptosfmt --stdin | rustfmt
