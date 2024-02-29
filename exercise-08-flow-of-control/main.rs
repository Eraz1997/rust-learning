fn main () {
    /*

    FLOW OF CONTROL

     */

    // If
    if 5 == 2 {
        //
    } else if 1 == 3 {
        //
    } else {
        //
    }

    // It is an expression and can return a value
    let z = if 1 == 2 {
        "hello"
    } else {
        "bye"
    };

    // Loop => while (true)
    let mut count = 1;
    loop {
        count += 1;
        if count == 3 {
            continue;
        }
        count += 1;
        if count >= 5 {
            break;
        }
    }

    'outer: loop { // You can label loops
        'inner: loop {
            break 'outer;
        }
    }

    let result = loop {
        break 1; // returns 1
    };

    // While
    while false {
        continue;
    }

    // For (loop through an `Iterator`)
    for i in 1..10 {} // 10 excluded
    for i in 1..=10 {} // 10 included

    // "Iterator" is a trait with the `iter`, `into_iter` and `iter_mut` functions
    let names = vec!["Bob", "Karl", "Arnold"];
    let mut mutable_names = vec!["Bob", "Karl", "Arnold"];
    for name in names.iter() { // This (read-only) borrows each item inside the loop, giving it back when it's done
        if name == &"Bob" { // The "&" is because the value is (read-only) borrowed
            println!("Here's Bob!");
        }
    }
    for name in mutable_names.iter_mut() { // This mutably borrows each item inside the loop, giving it back when it's done
        *name = if name == &mut "Bob" { // The "& mut" is because the value is mutably borrowed
            "Bob"
        } else {
            "Not Bob"
        }
    }
    for name in names.into_iter() { // This moves the ownership inside the for. At the end of the loop, the array doesn't exist anymore
        if name == "Not Bob" { // No need for any "&" because the loop owns the value
            println!("Here's not Bob!");
        }
    }
    // `names` doesn't exist anymore

    // Match
    //
    // Notes:
    // - Must be exhaustive
    let x = 5;
    match x {
        0 => println!("zero!"),
        1 | 5 | 8 => println!("Nice number"),
        number @ 15..=56=> println!("Not nice {number}"), // binding with "@"
        _ => println!("Nothing exceptional"),
    }

    match (1, 2, 3) { // Destructuring tuples
        (0, y, z) => println!("{y} {z}"),
        (1, _, 3) => println!("Starting with 1 and ending with 3"),
        (.., 3) => println!("Ending with 3"),
        _ => println!("Boh"),
    }
    // Arrays do the same, but you can also do this
    match [1, 2, 3, 4, 5] {
        [1, rest @ .., 5] => println!("{:?}", rest), // Store the "rest" like JS destructuring
        _ => (),
    }
    // Enums work as expected, including associated values like Swift's

    let referenced = 4;
    let reference = &referenced;
    match reference {
        &value => println!("{value}"), // Destructure
    }
    match *reference { // Dereference before matching
        value => println!("{value}"),
    }
    match referenced {
        ref reference if *reference > 16 => println!("{:?}>16", reference), // The "if" is a guard!
        ref reference=> println!("{:?}", reference), // "ref" creates a reference, "ref mut" for mutables
    }
    // Destructuring for structs works as you expect, with also nested destructuring

    // IF-LET
    //
    // if-let can be used to match enum values

    enum TestEnum {
        Foo,
        Bar,
    }
    let test_enum = TestEnum::Bar;
    if let TestEnum::Foo = test_enum {
        //
    }
    let test_optional = Some(1);
    if let Some(number) = test_optional { // Useful to unwrap optionals and similar!
        println!("{number}");
    }

    // LET-ELSE
    let Some(number) = test_optional else {
        panic!("Optional found!");
    }; // Unwraps and catches like Zig's catch

    // WHILE-LET
    //
    // Works as you expect
}