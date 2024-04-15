fn main () {
    /*
    RAII

    - Variable initialisation means also getting ownership of an object
    - When the object goes out of scope, it's destroyed and the ownership is lost
    - Not all variables own resources (e.g. references, "&", see "exercise-04-variables")
     */
    struct ToDrop;
    // "Drop" is the trait to implement destructors
    impl Drop for ToDrop {
        fn drop(&mut self) {
            println!("ToDrop is being dropped");
        }
    }
    let _to_drop = ToDrop;

    // "Box" allocates stuff in the heap. When the owning variable goes out of scope, memory is deallocated
    let _box1 = Box::new(3i32);
    // Content in the heap is accessed with the "*"
    println!("{}", *_box1);

    /*
    OWNERSHIP AND MOVES

    - Resources can only have one owner
    - Once moved/freed, variables are no more usable and their usage is prevented at compile-time!! :wow:
     */
    // Assignments by value are moves (like C++'s std::move)
    let _box2 = _box1; // _box1 is no more usable

    // Mutability can change when transferring ownership
    let mut _box3 = _box2;

    fn destroy_box(_box: Box<i32>) {}
    // Also passing arguments by value destroys variables, as the ownership is transferred inside the function scope
    destroy_box(_box3); // _box3 is no more usable

    // When destructuring objects, some fields can be moved and other can be referenced.
    use std::fmt::Debug;
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }
    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };
    let Person { name, ref age } = person; // person.name is no more usable, while person.age is.

    /*
    BORROWING

    - Variables are usually borrowed and not moved
    - Borrowing means referencing (&T) instead of assigning (T)
    - The compiler statically guarantees that references always point to valid objects: you can't
      free in-use objects
     */
    fn borrow(_: &Box<i32>) {}
    let _box4 = Box::new(2i32);
    {
        let _ref_to_box = &_box4;
        // destroy_box(_box4); // Compiler error: can't destroy here because a reference to _box4 is used later
        borrow(_ref_to_box); // No need to &_ref_to_box because it's already a reference
    }
    destroy_box(_box4); // Now you can destroy the box because all its references went out of scope

    // &mut mutably borrows resources
    let _box5 = Box::new(5i32);
    // let mut _ref_box5 = &mut _box5; // Can't do it because _box5 is immutable
    let mut _mutable_box5 = _box5;
    let mut _ref_box5 = &mut _mutable_box5; // Now I can

    // - Resources can be immutably borrowed infinite times
    // - While immutably borrowed, resources can't be mutably borrowed
    // - Only one mutable borrow can happen at a time
    // - Immutable borrows can happen again after mutable borrow's last usage

    // The "ref" keyword is the same as "&"
    let _box6 = Box::new(5i32);
    let ref _immutable_ref_box6_1 = _box6;
    let mut _mutable_box6 = _box6;
    let ref mut _mutable_ref_box6_1 = _mutable_box6;

    /*
    LIFETIMES

    - The lifetime of a resources starts when it creates and ends when it destroys
    - It relates to scopes, but it's not the same
    - They're usually omitted, but they can be explicitly set using generics
    - Ignoring elision, function signatures with lifetimes have a few constraints:
        - any reference must have an annotated lifetime.
        - any reference being returned must have the same lifetime as an input or be static.
    - Same holds for methods inside "impl" blocks
    - Lifetimes can be annotated inside structs as well, "impl" (as generics), traits
     */

    fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) { // 'a and 'b must both be at least as long as the function `print_refs`
        println!("x is {} and y is {}", x, y);
    }
    fn failed_borrow<'a>() { // A function which takes no arguments, but has a lifetime parameter `'a`.
        let _x = 12;
        // let _y: &'a i32 = &_x; // ERROR: `_x` does not live long enough - it was created later than failed_borrow
    }
    let (four, nine) = (4, 9);
    print_refs(&four, &nine); // The lifetime of `four` and `nine` must be longer than that of `print_refs`.
    failed_borrow(); // Because the lifetime is never constrained, it defaults to `'static`.

    // You can use bounds as for generics
    fn print_ref<'a, T>(t: &'a T) where
        T: Debug + 'a { // T must implement debug and must outlive "'a"
        println!("`print_ref`: t is {:?}", t);
    }
    // Longer lifetimes can be coerced into shorted ones
    fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 { // 'a must outlive 'b and is coerced to 'b in the return type
        first
    }

    // 'static
    // - As a reference lifetime: it means that the variable lives until the end of the program (no constraints on the creation)
    // - As a trait bound: it means the type does not contain any non-static references.
    // They can be coerced to shorter lifetimes

    let _s: &'static str = "hello world"; // as a reference lifetime
    static NUM: i32 = 18; // it's a constant with static lifetime (more elegant :wow:)

    // the function can hold on to the type for as long as they want and it will never become invalid until they drop it
    fn print_it( input: impl Debug + 'static ) {}
    let i = 5;
    print_it(i); // i is owned and contains no references, thus it's 'static:
    // print_it(&i); // &i only has the lifetime defined by the scope of main(), so it's not 'static:
}