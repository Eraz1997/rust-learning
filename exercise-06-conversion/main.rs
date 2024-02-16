fn main () {
    /*

    CONVERSION

    Notes:
    - It's casting/converting for custom types (structs and enums)
    - It leverages traits
     */

    // "From" trait
    use std::convert::{From, Into};

    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item } // Note the missing ";" to return the value
        }
    }

    let number = Number::from(2);

    // "Into" trait

    // "impl Into<Number> for i32" is automatically inherited from "impl From<i32> for Number"!!

    impl Into<i32> for Number {
        fn into(self) -> i32 {
            self.value
        }
    }

    let number2: Number = 12.into();
    let integer: i32 = number2.into();

    // TryFrom and TryInto return "Result" (fallible)
    use std::convert::TryFrom;

    struct EvenNumber (i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(item: i32) -> Result<Self, Self::Error> {
            if item % 2 == 0 {
                Ok(EvenNumber(item))
            } else {
                Err(())
            }
        }
    }

    /*
    STRING CONVERSIONS

    Notes:
    - Trait "ToString" exists and requires "to_string" method to be implemented
    - It's recommended to implement "fmt::Display" instead, which automatically implements "ToString" trait and "fmt" method
    - Trait "FromStr" can be implemented to get "parse" method functionality
     */

    use std::fmt;

    struct Circle {
        radius: f32,
    }

    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Circle of radius {}", self.radius)
        }
    }

    let circle = Circle { radius: 1.0 };
    let circle_string = circle.to_string();

    // "FromStr" is already implemented for many standard types
    let parsed: i32 = "5".parse().unwrap();
    let parsed2 = "5".parse::<i32>().unwrap();
}