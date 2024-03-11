#![allow(unused_varaibles)] // Inner attribute: it applies to the enclosing item (the crate, in this case)

// Attributes are also used to define crate metadata
#![crate_type = "lib"]
#![crate_name = "test_lib"]

fn main () {
    /*

    ATTRIBUTES

    - Metadata applied to some module, item or crate
    - They can also take arguments with #[attribute = "value"], #[attribute(key = "value")] or #[attribute(value)]
    - #[allow(dead_code)] is an example
     */

    #[derive(Debug)] // Outer attribute: it applies to the item following it
    struct Point(i32, i32);

    /*
    CFG

    - Used to conditionally compile code based on configuration like architecture and OS
    - Runtime checks are done with cfg macro instead
     */

    #[cfg(target_os = "linux")]
    fn linux_only() {
        println!("Linux only");
    }

    #[cfg(not(target_os = "linux"))]
    fn not_linux_only() {
        println!("not linux only");
    }

    if cfg!(target_os = "linux") {
        println!("Linux!");
    }

    // Custom conditionals are passed to rustc with --cfg
    #[cfg(environment = "dev")]
    fn dev_func() {
        println!("This is a dev env");
    }

    #[cfg(environment = "dev")]
    dev_func();
}