fn main () {
    /*

    PRIMITIVES

    Notes:

    Integers: i8, i16, i32, i64, i128, isize (pointer size)
    Same for u
    Float: f32, f64
    char
    bool
    Empty tuple (void): ()

    Arrays: [1,2,3]
    Tuples: (1, true)

     */

    /*

    TUPLES

     */

    fn example() -> (bool, u8) {
        (false, 2)
    }

    println!("First element: {}", example().0);


    /*

    ARRAYS

    Notes:

    Arrays length is known at compile time. They reside in the stack.
    Slices length is not known at compile time.

     */

    let example_array: [i8; 3] = [1, 2, 3]; // The signature of the array is [Type; Length]
    let zeroed_array: [i8; 500] = [0; 500]; // This way you create an array of 500 zeroes.

    println!("Lengths: {}, {}", example_array.len(), zeroed_array.len());

    fn print_length(array: &[i8]) { // This type is the slice, it doesn't contain the length
                                    // This function borrows an array as a slice
        println!("Length: {}", array.len());
    }

    print_length(&zeroed_array); // Borrow whole array as a slice
    print_length(&zeroed_array[1 .. 5]); // Borrow part of the array as a slice

    println!("{}", zeroed_array[5]); // Unsafe get
    match zeroed_array.get(5) { // Safe get, it returns an `Option`
        Some(value) => println!("{value}"),
        None => (),
    }
}