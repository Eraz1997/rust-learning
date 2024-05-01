fn main () {
    /*
    UNSAFE OPERATIONS

    Unsafe annotations in Rust are used to bypass protections put in place by the compiler
     */

    // Dereferencing raw pointers
    // Raw pointers * and references &T function similarly, but references are always safe because
    // they are guaranteed to point to valid data due to the borrow checker
    let raw_p: *const u32 = &10;
    unsafe {
        assert!(*raw_p == 10);
    }

    // Calling unsafe functions
    // Some functions can be declared as unsafe, meaning it is the programmer's responsibility to
    // ensure correctness instead of the compiler's
    use std::slice;
    let some_vector = vec![1, 2, 3, 4];
    let pointer = some_vector.as_ptr();
    let length = some_vector.len();
    unsafe {
        let my_slice: &[u32] = slice::from_raw_parts(pointer, length);
        assert_eq!(some_vector.as_slice(), my_slice);
    }

    // accessing or modifying static mutable variables
    // implementing unsafe traits

    /*
    INLINE ASSEMBLY
     */
    use std::arch::asm;

    let i: u64 = 3;
    let o: u64;
    unsafe {
        asm!(
        "mov {0}, {1}",
        "add {0}, 5",
        out(reg) o,
        in(reg) i,
        );
    }
    assert_eq!(o, 8);

    // or

    let mut x: u64 = 3;
    unsafe {
        asm!("add {0}, 5", inout(reg) x);
    }
    assert_eq!(x, 8);

    // or

    let x2: u64 = 3;
    let y: u64;
    unsafe {
        asm!("add {0}, 5", inout(reg) x2 => y);
    }
    assert_eq!(y, 8);

    // Skipped the rest, as it won't be needed, likely
}