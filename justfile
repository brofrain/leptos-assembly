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

# Runs front-end tests
test-client:
    cargo test --lib --features client

# Runs back-end tests
test-server:
    cargo test --features server

# Runs tests
test: test-client test-server

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
        rustfmt build.rs {{ flag }} &
        for f in `find src -name '*.rs'`; do
            rustfmt $f {{ flag }} &
        done
        wait
    )

# Formats Leptos components using leptosfmt
_fmt-leptosfmt flag='':
    leptosfmt src/components/**/*.rs {{ flag }}

# Formats Rust files including Leptos component syntax
fmt-rs:
    just _fmt-rustfmt
    just _fmt-leptosfmt

_prettier flag:
    npx prettier "(!(locales/TODO.*)/|!(pnpm-lock))*.{html,yaml,yml,toml}" {{ flag }}

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

# Runs check against the client-side
check-client:
    cargo check --lib --features client

# Runs check against the server-side
check-server:
    cargo check --features server

# Checks Rust codebase
check: check-client check-server

_clippy flag feature:
    cargo clippy {{ flag }} --features {{ feature }} -- -D warnings

# Lints Rust front-end codebase with Clippy
lint-client:
    just _clippy --lib client

# Lints Rust back-end codebase with Clippy
lint-server:
    just _clippy '' server

# Checks for TypeScript errors
lint-tsc:
    npx tsc

# Performs all lints
lint: lint-client lint-server lint-tsc

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
cargo-expand@1.0.74
cargo-outdated@0.13.1
cargo-audit@0.18.2
cargo-edit@0.12.2
''', '\s+', ' ')

# Performs full project setup
setup:
    #!/usr/bin/env sh
    (
        # Rust toolchain
        rustup toolchain install nightly --profile minimal -c rustfmt clippy
        rustup target add wasm32-unknown-unknown

        # Node dependencies
        # (should not run in parallel with other stuff,
        # because `playwright install-deps` can ask for permissions)
        npm i -g pnpm
        pnpm i --frozen-lockfile
        npx playwright install
        npx playwright install-deps

        # Cargo executables
        for dep in {{ CARGO_EXECUTABLES }}; do
            cargo install $dep &
        done

        wait
    )
    just clean

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
