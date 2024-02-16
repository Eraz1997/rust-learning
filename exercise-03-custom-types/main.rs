fn main () {
    /*

    CUSTOM TYPES

    - Structs
    - Enums

    Constants

     */

    // 3 types of structs
    struct Unit; // Unit struct (useful for generics)
    struct Pair(i32, u32); // Tuple struct
    struct Point {
        // Classic C structs
        x: i32,
        y: i32,
    }

    let unit = Unit; // Init unit struct

    let pair = Pair(1,2); // Init tuple struct
    println!("{}", pair.0); // access tuple struct fields

    let Pair(_, second_element) = pair; // JS-like destructure

    let x = 2;
    let point = Point { x, y: 2 }; // init with named fields or named variables
    println!("x {}", point.x); // access fields

    let second_point = Point { x: 1, ..point}; // spread in JS
    let Point { x, y: y_copy} = second_point; // destructure like JS

    // Destructuring can be nested!



    /*
    ENUMS

    Definition: types with different variants.
    Variant: basically, a struct (unit, tuple or named).
     */

    enum WebEvent {
        OnPageLoad, // unit structs
        DidPressKey(char), // tuple structs
        DidTap { x: i32, y: i32 }, // named structs
    }

    let event = WebEvent::DidTap { x: 1, y: 2 };

    type Ev = WebEvent; // Type Alias
    let event2 = Ev::OnPageLoad;

    // Swift-like extensions
    impl WebEvent {
        fn test(&self) {
            match self { // switch-case
                // USING THE SELF ALIAS!!!
                Self::OnPageLoad => println!("Page load!"),
                Self::DidPressKey(key) => println!("Key pressed: {key}"),
                Self::DidTap { x, y } => println!("Tapped at ({x}, {y})"),
            }
        }
    }

    // Scoping with "use"
    // use crate::WebEvent::{OnPageLoad, DidPressKey, DidTap};
    // or use WebEvent::*
    // It can be done only if the "WebEvent" enum is outside of main

    event.test();
    event2.test();

    // C-like structs with associated values
    enum Color {
        Red = 0xff0000, // if omitted, it auto-counts from 0 like in C
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }
    let color = Color::Red;
    println!("{:x}", color as i32);

    /*

    CONSTANTS

     */

    const THRESHOLD: i32 = 1; // Immutable constant
    static VALUE: i32 = 2; // Mutable variable with 'static lifetime (you'll see). It can be accessed or changed but it's unsafe
}