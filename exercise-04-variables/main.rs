fn main () {
    /*

    VARIABLES

     */

    let var1 = true; // Rust usually infers the type from the context without specifying it
    let immutable = 1; // Variables are immutable by default (const in JS)
    let mut mutable = 1; // "mut" makes them mutable (var in JS)

    {
        let scoped = 1; // Variables are scoped. This doesn't exist outside this anonymous block
        let var1 = 1; // Inner-scoped variables can shadow outer-scoped ones
        let mutable = mutable; // Shadowing a mutable variable by copying it to a homonym one makes it immutable in the current scope
    }

    let uninitialised; // You can declare first, without initialising, but it's highly discouraged.
    uninitialised = 1;
}