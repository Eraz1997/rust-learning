fn main () {
    /*
    TESTING

    - Three types of testing: Unit, Doc, Integration
     */

    // Unit testing
    // Run with `cargo test` (if you have a Cargo.toml file)
    fn add(x: i32, y: i32) -> i32 { x + y }
    fn panicking_fn() { panic!("aaaaa") }
    #[cfg(test)]
    mod tests {
        // Note this useful idiom: importing names from outer (for mod tests) scope.
        use super::*;

        #[test]
        fn test_add() {
            assert_eq!(add(1, 2), 3);
        }

        // They can also have a Result return type
        #[test]
        #[ignore] // skip
        fn test_add_2() -> Result<(), String> {
            assert_eq!(add(2, 3), 5);
            Ok(())
        }

        #[test]
        #[should_panic] // tests that a function panics, with optional `expect = "error message"` arg
        fn test_should_panic() {
            panicking_fn();
        }

        // DOCUMENTATION TESTING
        /// First line is a short summary describing function.
        ///
        /// The next lines present detailed documentation. Code blocks start with
        /// triple backquotes and have implicit `fn main()` inside
        /// and `extern crate <cratename>`. Assume we're testing `doccomments` crate:
        ///
        /// ```
        /// let result = doccomments::add2(2, 3);
        /// assert_eq!(result, 5);
        /// ```
        pub fn add2(a: i32, b: i32) -> i32 {
            a + b
        }

        /// Usually doc comments may include sections "Examples", "Panics" and "Failures".
        ///
        /// The next function divides two numbers.
        ///
        /// # Examples
        ///
        /// ```
        /// let result = doccomments::div(10, 2);
        /// assert_eq!(result, 5);
        /// ```
        ///
        /// # Panics
        ///
        /// The function panics if the second argument is zero.
        ///
        /// ```rust,should_panic
        /// // panics on division by zero
        /// doccomments::div(10, 0);
        /// ```
        pub fn div(a: i32, b: i32) -> i32 {
            if b == 0 {
                panic!("Divide-by-zero error");
            }

            a / b
        }
        // Code blocks in documentation are automatically tested when running the regular cargo test command

        // INTEGRATION TESTS
        // Similar to unit tests, but they test methods from other crates
    }
}