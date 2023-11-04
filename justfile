_default: help

# Prints avialable recipes
@help:
    echo "\nRun a task using \`just [RECIPE]\`."
    just --list

# --- Build & serve ---

# Clears build artifacts
clean:
    cargo clean

# Builds the application in release mode
build:
    cargo leptos build --release

# Runs development server and watches for changes
dev:
    cargo leptos watch --hot-reload

# Runs development server
serve:
    cargo leptos serve

# Serves the application in release mode
serve-release:
    cargo leptos serve --release

# --- Test ---

# Runs tests
test:
    # cargo-nextest doesnt't support doctests yet: https://github.com/nextest-rs/nextest/issues/16
    cargo test --doc
    # TODO add some example dummy tests
    cargo nextest run

# Serves the app and runs E2E tests with Playwright
e2e:
    npx playwright test

# Serves the app in release mode and runs E2E tests with Playwright
e2e-release:
    PLAYWRIGHT_WEBSERVER_RELEASE_MODE=true just e2e

# Serves the app and opens Playwright UI
e2e-ui:
    npx playwright test --ui

# Serves the app in release mode and opens Playwright UI
e2e-ui-release:
    PLAYWRIGHT_WEBSERVER_RELEASE_MODE=true just e2e-ui

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

# Lints the project, but disallows warnings
lint-ci:
    cargo clippy -- -D warnings
    just lint-ts

# --- Security ---

# Runs Cargo dependency audit
audit-cargo:
    cargo audit

# Runs Node dependency audit
audit-node:
    pnpm audit --prod

# Runs all dependency audits
audit: audit-cargo audit-node

# Checks for outdated Cargo dependencies
outdated-cargo:
    cargo outdated --root-deps-only --exit-code 1

# Checks for outdated Node dependencies
outdated-node:
    pnpm outdated

# Checks for outdated dependencies
outdated: outdated-cargo outdated-node

# --- Dependency management ---

CARGO_EXECUTABLES := replace_regex('''
just@1.15.0
cargo-leptos@0.2.0
leptosfmt@0.1.17
cargo-nextest@0.9.61
cargo-outdated@0.13.1
cargo-audit@0.18.2
''', '\s+', ' ')
CARGO_DEV_EXECUTABLES := replace_regex('''
cargo-expand@1.0.74
cargo-edit@0.12.2
''', '\s+', ' ')

_setup +executables:
    #!/usr/bin/env sh
    (
        # Rust toolchain
        rustup toolchain install nightly --profile minimal -c rustfmt clippy
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
        npx playwright install
        npx playwright install-deps

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

_upgrade:
    cargo upgrade

_upgrade-latest:
    cargo upgrade --incompatible allow

_update:
    cargo update

# Updates Cargo packages to their latest versions in the specified ranges
update-cargo: _upgrade _update

# Updates Node packages to their latest versions in the specified ranges
update-node:
    pnpm update

# Updates packages to their latest versions in the specified ranges
update: update-cargo update-node

# Updates Cargo packages to their latest versions ignoring ranges in Cargo.toml
update-cargo-latest: _upgrade-latest _update

# Updates Node packages to their latest versions ignoring ranges in package.json
update-node-latest:
    pnpm update --latest

# Updates packages to the latest versions ignoring their ranges
update-latest: update-cargo-latest update-node-latest

_vscode-fmt:
    # Using `leptosfmt --stdin --rustfmt` seems to add redundant newlines
    leptosfmt --stdin | rustfmt
