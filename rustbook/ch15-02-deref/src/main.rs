use std::ops::Deref;

fn main() {
    // Simple example showing how we can create a reference to a value
    // and use the dereference operator to access the value of the
    // reference
    let x = 5;
    let y = &x; // creates a reference to x
    assert_eq!(5, x);
    assert_eq!(5, *y); // dereferences the pointer

    // This example uses Box<T> to create a reference instead of
    // directly creating a reference. We again need to use the
    // dereference operator to access the underlying value
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // This example uses a type that implements the Deref trait. We end
    // doing the exact same as Box or an & reference.
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Deref coercsion happens twice, by converting &MyBox<String> into
    // &String, and then converting &String into &str.
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // If we didn't have deref coercion we would have to do this...
    hello(&(*m)[..]);
}

// Defines a custom version of Box, note that it won't allocate onto the
// heap like normal box does though. This type is a tuple struct that
// implements a single value of type T.
struct MyBox<T>(T);

// Implements a new method simpliar to Box<T>
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Implements the std::ops::Deref trait that
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
