fn main() {
    /*

    CARGO

    - Official Rust package manager
    - Dependency management and integration with crates.io, which is the official package registry
    - Unit tests
    - Benchmarks
     */

    /*
    Dependencies

    - Create new executable project with "cargo new [...]"
    - Create new library project with "cargo new --lib [...]"
    - Add a dependency under "[dependencies]" in Cargo.toml or with "cargo add"
        - e.g. "clap" is the most used library for CLIs
    - Build with "cargo build" and run with "cargo run"
     */

    /*
    Conventions

    - To have more than one executable per project, place files under the "bin" directory
    - You can build them independently with "--bin" or build them all
    - You have to specify "--bin" with "cargo run"
     */

    /*
    Tests

    - Test with "cargo test"
    - Place integration tests for your library (tests which call your library as if they were integrations) in the "tests" folder
    - Tests run concurrently, make sure they don't have race conditions
     */

    /*
    Build scripts

    - If present, "build.rs" will be compiled and run before compiling the app/lib
    - It can be overridden with the "build = [...].rs" config line under "[package]" in Cargo.toml
     */
}