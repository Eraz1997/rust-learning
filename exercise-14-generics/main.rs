use std::fmt::{Debug, Display};

fn main () {
    /*

    GENERICS

     */

    // Generic struct
    struct SingleParamStruct<T>(T);
    let _s = SingleParamStruct('a');

    // Generic function
    fn generic_1<T>(_s: T) {}
    generic_1("lala");
    fn generic_2<T>(_s: SingleParamStruct<T>) {}
    generic_2(SingleParamStruct(1));

    // Implementations
    impl SingleParamStruct<i32> {
        fn i32_function(&self) {}
    }
    let i32_s = SingleParamStruct(12);
    i32_s.i32_function();
    impl<T> SingleParamStruct<T> {
        fn generic_function(&self, _x: T) {}
    }
    i32_s.generic_function(12);

    // Traits
    trait MyTrait<T> {
        fn my_function(&self, _: T);
    }
    // Implement `MyTrait<T>` for any generic parameter `T` and caller `U`.
    impl<T, U> MyTrait<T> for U {
        fn my_function(&self, _: T) {}
    }

    // Bounds (used to access methods/properties/features of a given trait and exclude other types at compile time)
    fn printer<T: Display>(t: T) {
        println!("{}", t);
    }
    fn all_printers<T: Display + Debug>(t: T) {
        println!("Display: {}", t);
        println!("Debug: {:?}", t);
    }

    // Where clause
    trait PrintInOption {
        fn print_in_option(self);
    }
    impl<T> PrintInOption for T where Option<T>: Debug {
        fn print_in_option(self) {
            println!("{:?}", Some(self));
        }
    }

    // Associated types (in traits)
    // They make you go from this
    trait Contains1<A, B> {
        fn contains(&self, _: A, _: B) -> bool;
    }
    fn difference1<A, B, C>(container: &C) -> i32 where C: Contains1<A, B> { 1 }
    // To this
    trait Contains2 {
        type A;
        type B;
        fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    }
    fn difference2<C: Contains2>(container: &C) -> i32 { 1 }
}