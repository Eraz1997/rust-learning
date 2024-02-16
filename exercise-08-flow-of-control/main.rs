fn main () {
    /*

    FLOW OF CONTROL

     */

    // If
    if 5 == 2 {
        //
    } else if 1 == 3 {
        //
    } else {
        //
    }

    // It is an expression and can return a value
    let z = if 1 == 2 {
        "hello"
    } else {
        "bye"
    };

    // Loop => while (true)
    let mut count = 1;
    loop {
        count += 1;
        if count == 3 {
            continue;
        }
        count += 1;
        if count >= 5 {
            break;
        }
    }

    'outer: loop { // You can label loops
        'inner: loop {
            break 'outer;
        }
    }

    let result = loop {
        break 1; // returns 1
    };

    // While
    while false {
        continue;
    }
}