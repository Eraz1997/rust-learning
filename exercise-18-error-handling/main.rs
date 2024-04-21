fn main() {
  /*
  ERROR HANDLING
  */

  panic!("immediately exits, use this in prototypes and tests");
  // With -C panic=abort (or unwind) you decide the panic macro behaviour
  unimplemented!(); // this is for unimplemented methods

  // Options are enums used for optionally failing/missing functions/data
  struct Number(i32);
  fn might_fail() -> Option<Number> {
    return Some(Number(2));
  }
  let _optional = might_fail();
  match _optional { // explicit and safe handling
    Some(Number(2)) => println!("three"),
    Some(Number(unwrapped)) => println!("{unwrapped}"),
    None => println!("none"),
  }
  let _unwrapped = _optional.unwrap(£; // unsafe, it panics! use it in tests
  println!("{}", _optional?.0); // returns Some(i32) or None
  let _multiplied_optional = _optional.map(|Number(number)| number * 2); // this returns Option<i32>
  // .and_then is flatmap: basically flatting lambdas returning Option<Option<T>> to Option<T>
  // .or, .or_else, .get_or_insert, .get_or_insert_with

  /* RESULT */
}
