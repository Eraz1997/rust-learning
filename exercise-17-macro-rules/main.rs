fn main() {
    /*
    MACROS

    - They allow metaprogramming
    - They are expanded to abstract syntax trees, not plain strings (like C), so you don't get unexpected bugs
    - They are expanded to source code
    - Params have the $ prefix and are of type
        block
        expr is used for expressions
        ident is used for variable/function names
        item
        literal is used for literal constants
        pat (pattern)
        path
        stmt (statement)
        tt (token tree)
        ty (type)
        vis (visibility qualifier)
     */

    macro_rules! create_function {
        ($func_name: ident, $expression: expr) => {
            fn $func_name() {
                // The `stringify!` macro converts an `ident` into a string.
                println!("{:?} goes {:?}", stringify!($expression), $expression);
            }
        };

        // Arguments don't need comma separation, they can be separated by anything
        // And they also allow overloading
        (eval $func_name: ident = $expression: expr) => {
            fn $func_name() {
                // The `stringify!` macro converts an `ident` into a string.
                println!("{:?} = {:?}", stringify!($expression), $expression);
            }
        };
    }
    create_function!(func1, 1 + 4);
    create_function!(eval func2 = 1 + 4);
    func1();
    func2();

    // $(...),+ means that you will have 1 or more repeating arguments, while $(...),* means 0 or more
    macro_rules! find_min {
        ($x:expr) => ($x);
        ($x:expr, $($y:expr),+) => {
            // This is called DSL (domain specific language)
            {
                let rest = find_min!($($y),+);
                if $x < rest {
                    $x
                } else {
                    rest
                }
            }
        }
    }
    find_min!(1, 3, 5, 0, 10);
}