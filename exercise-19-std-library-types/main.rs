fn main() {
    /*
    STD LIBRARY TYPES
     */

    // Variables are allocated on the stack by default
    // Use Box<T> to allocate variables on the heap
    // - When it goes out of scope, it destroys and frees the memory
    // - The "*" lets you access the referenced value (you can have multiple wrapping Boxes, so "***...")
    let _boxed = Box::new(2);
    *_boxed;

    // Vec<T> are variable-sized vectors. They have a pointer, a length and a capacity (once reached, the vector is reallocated)
    let mut _vector: Vec<i32> = vec![1, 2, 3]; // the vec macro creates a Vec<T> object
    _vector.push(4);
    _vector.len();
    _vector.pop();
    for (_index, _item) in _vector.iter().enumerate() {} // iter, iter_mut, iter_into

    // Strings
    // `String` is a Vec<u8>
    // `&str` is a slice of u8 (`&[u8]`), hence a variable-length pointer to a valid UTF-8 sequence
    //
    let long_string = "\u{123D} inserts a Unicode character. \x52 inserts bytes. String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    let quotes = r#"And then I said: "There is no escape!""#;
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    let bytestring: &[u8; 21] = b"this is a byte string";
    use std::str;
    if let Ok(my_str) = str::from_utf8(bytestring) {
        println!("And the same as text: '{}'", my_str);
    }

    // HashMap<K, V>
    use std::collections::HashMap;
    let mut contacts = HashMap::new();
    contacts.insert("Daniel", "798-1364");
    if let Some(number) = contacts.get(&"Daniel") {
        println!("Calling Daniel: {}", number);
    }
    contacts.remove(&"Ashley");
    // You can create your custom key type
    #[derive(PartialEq, Eq, Hash)]
    struct MyKey(i32);

    // HashSet<T> is HashMap<T, ()>

    // Rc is a reference counter:
    // - It allows multiple owners of a resource
    // - It increases the count by 1 every time it's cloned and decreases by 1 every time it's destroyed
    use std::rc::Rc;
    let rc_examples = "Rc examples".to_string();
    {
        let rc_a: Rc<String> = Rc::new(rc_examples); // count is 1 and `rc_example` is moved so no more usable
        {
            let rc_b = Rc::clone(&rc_a); // count is 2
        }
        // count is 1
    }
    // count is 0 and nothing is usable anymore

    // std::sync::Arc is the same for threads, as it counts on the heap
}