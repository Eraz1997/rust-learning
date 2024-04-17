fn main () {
    /*
    TRAITS

    - They are definitions of collections of methods (aka interfaces of types)
     */

    trait Animal {
        fn name(&self) -> &'static str;
        fn noise(&self) -> &'static str;
    }
    struct Sheep;
    impl Animal for Sheep {
        fn name(&self) -> &'static str { "sheep" }
        fn noise(&self) -> &'static str { "beeeh" }
    }
    let sheep = Sheep;
    println!("{} says: '{}'", sheep.name(), sheep.noise());

    /*
    You can use #[derive(traits...)] to make the compiler auto-implement some standard traits

    Examples: Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default, Debug

    You can still decide to `impl Eq for YourStruct`
     */

    // Since return types must be concrete, as the compiler needs to know how much memory to allocate,
    // Returning a Box with a "dyn YourStruct" type is the way to generically return traits
    fn animal_factory() -> Box<dyn Animal> { Box::new(Sheep) }
    // or more elegantly, you can just return "impl Trait" to avoid using the heap
    fn elegant_animal_factory() -> impl Animal { Sheep }
    // You can also use it as a parameter type
    fn elegant_animal_consumer(_: impl Animal) {}
    elegant_animal_consumer(elegant_animal_factory());

    // Operator overloading is done with std::ops traits!
    use std::ops;
    struct MyInt(i32);
    impl ops::Add<MyInt> for MyInt {
        type Output = MyInt;
        fn add(self, rhs: MyInt) -> MyInt { MyInt(self.0 + rhs.0) }
    }
    MyInt(1) + MyInt(2);

    /*
    Other interesting basic traits are: Drop, Iterator
     */

    // Supertraits (there's no inheritance, but you can have superset traits)
    trait WalkingAnimal: Animal {}

    /*
    Ambiguity

    If your struct implements two traits having homonym methods (e.g. "get"), you can call them
    like

    <YourStructType as Trait1>::get(&your_struct_object);
     */
}