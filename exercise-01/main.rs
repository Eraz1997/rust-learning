// Single-line comment
/* Multi-line comments */

/// Library docs
#[allow(dead_code)] // this disables the warning "unused function" for this function
fn documented_function() {}

fn main() {
    /* 
    
    FORMATTED PRINT

    Notes:
    println uses the fmt::Display trait to display things. Structs which don't implements it can't be printed.
    
    */

    print!("hey Joe!\n"); // No newline
    println!("hey Bob!"); // Newline

    eprint!("Error 1\n"); // Error without newline
    eprintln!("Error 2"); // Error with newline

    println!("{}", 31); // Simple formatting
    println!("{0} + {2} = {1}", 1, 3, 2); // Formatting with indices
    println!("{name} {surname} is a {position}", name="Paolo", surname="Meneguzzi", position="Backend Engineer"); // Formatting with names
    println!("{0:b}, {0:o}, {0:x}, {0:X}", 123); // Formatting numbers in different bases

    let number: i32 = 142;
    println!("Captured {number}"); // Formatting by capturing variables in the scope

    println!("{:>8}", 1); // Left padding with spaces
    println!("{:0>8}", 1); // Left padding with zeroes
    println!("{:<8}", 1); // Right padding with spaces
    println!("{:<width$}", 1, width=4); // Left padding with spaces and named argument for width

    println!("{:.2}", 1.6314); // Format with fixed decimals

    let string: String = format!("{}", "text"); // Format to string
    println!("{string}");

    /*
    
    DEBUG TRAIT

    */

    #[derive(Debug)]
    struct DebugPrintable(i32); // Every type can automatically derive the Debug trait!
    println!("{:?}", DebugPrintable(12)); // Debug printing with "?"
    println!("{:#?}", DebugPrintable(12)); // Pretty debug printing with "#?"

    /*
    
    DISPLAY TRAIT
    
    */

    use std::fmt; // Import the "std::fmt" library

    struct PrintableStruct(i32);

    impl fmt::Display for PrintableStruct {
        // Make PrintableStruct printable by implementing this specific formatting function
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            // Writes the first (and only one) element of the struct to the formatter
            // If you omit the ";", the write macro returns a fmt::Result
            write!(formatter, "{}", self.0)
        }
    }

    println!("{}", PrintableStruct(2));

    struct PrintableList(Vec<i32>);

    impl fmt::Display for PrintableList {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "[")?; // The "?" character prevents the macro from returning the fmt::Result. It returns only in case of errors.
            let vector = &self.0;
            for (count, value) in vector.iter().enumerate() {
                if count != 0 {
                    write!(formatter, ", ")?;
                }
                write!(formatter, "{value}")?;
            }
            write!(formatter, "]") // Finally, omit the "?" and ";" to return the fmt::Result
        }
    }

    println!("{}", PrintableList(vec![1,2,3,4,5]));

    /*
    
    OTHER FORMATTING TRAITS

    */

    struct Pixel {
        red: u32,
        green: u32,
        blue: u32,
    }

    impl fmt::Display for Pixel {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "[R: {}, G: {}, B: {}]", self.red, self.green, self.blue)
        }
    }

    impl fmt::Binary for Pixel {
        // Implement traits like "Binary" to enable binary formatting for the given structure.
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            let value: u32 = (self.red * 65536) + (self.green * 256) + self.blue;
            fmt::Binary::fmt(&value, formatter) // Again, omit the ";" to return. Delegate the implementation to the u32 implementation of the trait.
        }
    }

    println!("{}", Pixel { red: 10, green: 100, blue: 200});
    println!("{:b}", Pixel { red: 10, green: 100, blue: 200});
}
