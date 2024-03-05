fn main() {
    /*

    CRATES

    Notes:
    - A crate is a compilation unit
    - A crate can be compiled into a binary (default) or library (with rustc --crate-type=lib)
    - When you run "rustc main.rs", Rust creates a crate for "main.rs"
     */

    /*

    Libraries

    - Compile with "rustc --crate-type=lib lib.rs", you will get a lib[...].rlib file
    - You can tweak the crate name with "--crate-name"
    - Now you can compile this file with "rustc main.rs -o main --extern lib=liblib.rlib"
     */

    lib::public_function();
}