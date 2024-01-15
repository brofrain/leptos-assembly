// HACK
// As of cargo-leptos 0.2.2, it's not possible to build a SPA without a backend
// as well. The valid workarounds are:
// - Build an empty backend and never deploy it
// - Create a frontend build entrypoint for Trunk
// - Build the frontend crate and optimize wasm manually
// We choose the first option here, as it introduces no new CLI tools to the
// project. Everything is bundled and optimized by cargo-leptos, thus
// maintaining the workaround should be the least painful.
pub const fn main() {}
