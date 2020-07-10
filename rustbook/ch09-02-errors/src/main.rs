use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    // Result<T, E> will return either the value or an error. We can use a match
    // expression to determine what the result was and cascade through error
    // logic. There are cleaner ways to write this code using closures however.
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(err) => panic!("Problem creating the file {:?}", err),
            },
            other => panic!("Problem opening the file {:?}", other),
        },
    };
    println!("file: {:?}", f);

    // You can use the unwrap method, which is shorthand for calling the panic!
    // macro as was shown above. In practice you would likely use another form
    // of unwrap.
    let f = File::open("hello.txt").unwrap();
    println!("file: {:?}", f);

    // You can also use the expect method on a Result<T, E> which essentially
    // call the panic! macro but includes a customized message that helps
    // assist with debugging
    let f = File::open("hello.txt").expect("Opening hello2.txt failed!");
    println!("file: {:?}", f);

    // Next we will read the contents of a file and propogate any errors from
    // the call to that file
    let s = read_file_contents().unwrap();
    println!("contents: {}", s);

    // We can use a shorter version using the ? operator
    let s = read_file_contents_shorter().unwrap();
    println!("content: {}", s);
}

/// Propagates any errors to the caller and returns the contents of a file
fn read_file_contents() -> Result<String, io::Error> {
    let mut f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

/// This version does the same as above but uses the ? operator to propagate the
/// error up the call stack.
fn read_file_contents_shorter() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
