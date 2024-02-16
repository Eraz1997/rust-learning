fn main () {
    /*

    TYPES

     */

    /*
    CASTING
     */

    // There is no implicit casting in Rust!
    let decimal = 65.1_f64;
    let integer = decimal as u8;    // Explicit casting with "as"
    let character = integer as char;    // You cannot convert float to char directly

    // Safety and overflow checks are applied to casts. If you want to skip them:
    unsafe {
        let integer = decimal.to_int_unchecked::<u8>();
    }

    /*
    LITERALS
     */

    let var1 = 1u16; // Type annotation with type suffix in literals
    let var2 = 1.0; // Otherwise, type is guessed
    let var3 = 1;
    // Inference works not only at assignment time, but also by checking how the variable is used.

    use std;

    println!("{} vs {}", std::mem::size_of_val(&var2), std::mem::size_of_val(&var3)); // You can check the memory size of a variable in bytes

    /*
    ALIASES
     */
    type Inch = u64; // Type alias
}