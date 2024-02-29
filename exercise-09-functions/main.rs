fn main() {
    /*

    FUNCTIONS

     */

    let even = is_even(2);

    // ASSOCIATED FUNCTIONS AND METHODS
    struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        fn origin() -> Point { // Associated function ("static function")
            Point { x: 0, y: 0 }
        }

        fn double_x(&self) -> i32 { // Method
            self.x * 2
        }

        fn double_y_set(&mut self) { // Mutable method
            self.y *= 2;
        }

        fn destroy(self) { // Dereferencing method
            let Point { y, x} = self;
            // After this call, the Point object won't exist anymore in the caller's scope
        }
    }

    // CLOSURES
    let closure = |number| number * 2;
    closure(2);

    let color = String::from("green");

    // Closures borrow variables from the environment the less strict way between
    // 1. Immutable borrow (&T)
    // 2. Mutable borrow (&mut T)
    // 3. Taking ownership (T)

    // If you don't modify the variable, then it goes for 1
    let immutable_borrow_closure = || println!("`color`: {}", color);
    immutable_borrow_closure();
    let _reborrowed = &color; // In this case you can immutably reborrow the variable
    immutable_borrow_closure();
    let _moved = color; // But you can move or mutably reborrow after the last call of the closure

    let mut count = 0;
    // If you modify the variable, it tries to go for 2
    let mut mutable_borrow_closure = || { count += 1 };
    mutable_borrow_closure();
    mutable_borrow_closure();
    let _mutably_reborrowed = &mut count; // In this case you cannot reborrow before the last call to the closure

    // In other cases or if you specify "move", the variable is captured by value, namely moved inside the closure and cannot be used outside anymore
    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));
    // Can't use "haystack"

    let test_closure = |number: i32| println!("test {number}");

    // Fn is the type for immutably borrowing closures
    fn fn_parameter_test<Function>(function: Function) where Function: Fn(i32) -> bool {
        function(12);
    }
    // FnMut is the type for mutably borrowing closures
    fn fn_mut_parameter_test<Function>(mut function: Function) where Function: FnMut(i32) {
        function(12);
    }
    // FnOnce is the type for moving closures
    fn fn_once_parameter_test<Function>(function: Function) where Function: Fn(i32) {
        function(12);
    }
    // Even though you can declare a closure as FnOnce or FnMut, Rust still borrows variables the less strict way anyway

    // Why using generics?
    // Because Fn, FnMut and FnOnce are `traits` automatically implemented by Rust when you declare closures.
    // You need to take a generic argument implementing the right trait then.

    fn other_function_with_closure<Function: Fn()>(function: Function) { // you can be more concise with this syntax
        function();
    }

    fn_parameter_test(is_even); // Functions implement the Fn, FnMut and/or FnOnce traits as well!

    // You can also make functions return closures
    fn function_factory() -> impl Fn() { // You need to return a generic type implementing the Fn trait
        let text = "Fn".to_owned();
        move || println!("name: {text}") // You need to capture by value (T) because things on the stack go out of scope after the function call
    }

    // HIGH-ORDER FUNCTIONS

    // Functions that take one or more functions and/or produce a more useful function
    // Examples
    let upper = 1000;
    let sum_of_squared_odd_numbers: i32 =
        (0..).map(|n| n * n)                             // All natural numbers squared
            .take_while(|&n_squared| n_squared < upper) // Below upper limit
            .filter(|&n_squared| is_even(n_squared))     // That are odd
            .sum();                                     // Sum them
    println!("functional style: {}", sum_of_squared_odd_numbers);

    // DIVERGING FUNCTIONS, aka never-returning functions

    fn diverging() -> ! { // The "!" means it's diverging
        panic!("Bye");
    }
    // The "continue" keyword in a "match" statement is a diverging expression
}

fn is_even(number: i32) -> bool {
    number % 2 == 0 // You can also use the "return" statement
}