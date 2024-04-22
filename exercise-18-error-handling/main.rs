fn main() {
  /*
  ERROR HANDLING
  */

  panic!("immediately exits, use this in prototypes and tests");
  // With -C panic=abort (or unwind) you decide the panic macro behaviour
  unimplemented!(); // this is for unimplemented methods

  /* OPTIONS */
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
  let _unwrapped = _optional.unwrap(); // unsafe, it panics! use it in tests
  // _optional?.0 returns Some(i32) or terminates the current function returning None if it's None
  let _multiplied_optional = _optional.map(|Number(number)| number * 2); // this returns Option<i32>
  // .and_then is flatmap: basically flatting lambdas returning Option<Option<T>> to Option<T>
  // .or, .or_else, .get_or_insert, .get_or_insert_with

  /* RESULT */
  // Same as Option, but it carries a possible `Err` (which is a generic error)
  // You can use it the same way as you use Option
  use std::num::ParseIntError;
  let res: Result<i32, ParseIntError> = Ok(3);
  res.unwrap(); // panics if Err
  match res {
    Ok(2) => {},
    Ok(inner) => {},
    Err(error) => {}
  }
  res.and_then(|unwrapped| {
    let res2: Result<i32, ParseIntError> = Ok(3);
    res2.map(|unwrapped2| unwrapped * unwrapped2)
  }).map_err(|error| Some(2));
  // You can alias the Result to cast to specific errors
  type ParseResult<T> = Result<T, ParseIntError>;
  fn parse_test(string_to_parse: &str, string_to_parse2: &str) -> ParseResult<i32> {
    let first = string_to_parse.parse::<i32>()?; // The "?" here makes the whole function return the error if failing
    let second = string_to_parse2.parse::<i32>()?; // The "?" here makes the whole function return the error if failing
    Ok(first * second)
  }
  // You can also do this for simplicity
  #[derive(Debug, Clone)]
  struct DoubleError; // define error type
  // type Result<T> = std::result::Result<T, DoubleError>;
}
