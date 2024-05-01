fn main () {
    /*
    THREADS
     */

    use std::thread;
    let mut threads = vec![];


    // CHANNELS can be used to send data across threads
    use std::sync::mpsc;
    let (sender, receiver) = mpsc::channel();

    for index in 0..10 {
        // spawn takes a moving closure as input - it takes ownership of the environment
        let thread_sender = sender.clone(); // the sender must be cloned as it will be moved to the thread
        threads.push(thread::spawn(move || { // It returns the thread handle
            let result = index * 10;
            thread_sender.send(result);
            result // It can return the result
        }));
        // index is unusable now
    }

    for _ in 0..10 {
        // The order depends on the threads schedule
        println!("{}", receiver.recv().unwrap()); // Returns Result<T, RecvError>
    }

    for child in threads {
        println!("{}", child.join().unwrap()); // join returns Result<T, Error>
        // These are ordered of course
    }

    /*
    PATHS

    - Path represents a filesystem path and can be used to interact with the filesystem itself
    - Path is immutable and can't be modified in-place
    - PathBuf is the mutable version
     */
    use std::path::Path;
    let path = Path::new(".");
    let _displayable = path.display(); // The `display` method returns a `Display`able structure
    let mut new_path = path.join("a").join("b"); // returns a `PathBuf` ("./a/b" or ".\a\b" depending on the OS)
    new_path.push("c"); // PathBuf can be modified in-place
    new_path.push("myfile.tar.gz");
    new_path.set_file_name("package.tgz"); // updates the file name of the `PathBuf`
    match new_path.to_str() { // Convert the `PathBuf` into a string slice
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }

    /*
    FILE

    - File is a file handler and allows interacting with files
     */
    use std::fs::File;
    use std::io::prelude::*;
    let path = Path::new("test.txt");
    let display = path.display();
    let mut file = match File::open(&path) { // `open` opens files in read-only mode
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(bytes_read) => print!("{} contains {} bytes:\n{}", display, bytes_read, content),
    }
    // `file` goes out of scope, and the "test.txt" file gets closed

    // File::create(&path) // it opens and overwrites a file in write-only mode
    // file.write_all(text.as_bytes()) // it writes bytes to file

    /*
    CHILD PROCESSES
     */
    use std::process::Command; // the overpowered version of Python's os.system
    let output = Command::new("rustc")
        .arg("--version")
        .output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });
    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        print!("rustc failed and stderr was:\n{}", s);
    }

    // You can also pipe stdin and stdout
    use std::process::Stdio;
    let process = match Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
        Err(why) => panic!("couldn't spawn wc: {}", why),
        Ok(process) => process,
    };
    match process.stdin.unwrap().write_all("the quick brown fox jumped over the lazy dog\n".as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}", why),
        Ok(_) => println!("sent pangram to wc"),
    }
    // Because `stdin` does not live after the above calls, it is `drop`ed, and the pipe is closed.
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}", why),
        Ok(_) => print!("wc responded with:\n{}", s),
    }
    // otherwise you could do `let status_code = process.wait().unwrap()`

    /*
    OTHER FILESYSTEM stuff
     */
    // OpenOptions::new().create(true).write(true).open(path) // "touch" file
    // fs::create_dir("a") // mkdir
    // fs::create_dir_all("a/c/d") // mkdir -p
    // unix::fs::symlink("../b.txt", "a/c/b.txt")
    // windows::fs::symlink_file("../b.txt", "a/c/b.txt")
    // fs::read_dir("a") // returns `io::Result<Vec<Path>>`
    // fs::remove_file
    // fs::remove_dir

    /*
    PROGRAM ARGUMENTS
     */
    use std::env;
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let number: i32 = match args[1].parse() {
                Ok(n) => n,
                Err(e) => panic!("{e}"),
            };
            println!("Your number is {number}");
        },
        _ => println!("Usage: {} <number>", args[0].parse::<String>().unwrap()),
    }

    /*
    FFI (from C libraries)
     */
    #[cfg(target_family = "windows")]
    #[link(name = "my_lib")] // the "link" attribute contains the name of the library
    extern {
        fn ccosf(x: f32) -> f32;
    }

    // Since calling foreign functions is considered unsafe, it's common to write safe wrappers around them.
    #[cfg(target_family = "windows")]
    fn cos(x: f32) -> f32 {
        unsafe { ccosf(x) }
    }

}