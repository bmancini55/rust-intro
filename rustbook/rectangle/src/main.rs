/// This is a simple struct that holds values associated with a Rectangle. We
/// add the Debug trait so that the Rectangle can be printed using debug
/// prints.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /// Example of an associated function that acts as a constructor. This
    /// method will be called using the struct name as a namespace
    ///
    /// # Example
    /// ```
    /// let square = Rectangle::square(10);
    /// ```
    fn square(side: u32) -> Rectangle {
        Rectangle {
            height: side,
            width: side,
        }
    }

    /// Example oof a method that borrows self instead of taking ownership or
    /// mutably borrowing self. This method simply uses the associated values
    /// in the struct.
    ///
    /// # Example
    /// ```
    /// let rect = Rectangle { width: 30, height: 50 };
    /// println1("Area rectangle: {}", rect.area());
    /// ```
    fn area(&self) -> u32 {
        self.width * self.height
    }

    /// Example of accepting other arguments in a method. This is similar to
    /// other method but includes another object that is also borrowed.
    ///
    /// # Example
    /// ```
    /// let rect1 = Rectangle { width: 30, height: 50 };
    /// let rect2 = Rectangle { width: 20, height: 40 };
    /// let rect3 = Rectangle { width: 40, height: 40 };
    ///
    /// println1("2 inside 1: {}", rect1.can_hold(&rect2));
    /// println1("3 inside 1: {}", rect1.can_hold(&rect3));
    /// ```
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 40,
    };

    // exercises the debug print
    println!("rect1 is {:#?}", rect1);

    // exercises the associated function that acts as a constructor
    println!("square {:#?}", Rectangle::square(10));

    // execises the method call
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // execises the method that borrows another rectangle
    println!("rect 2 fits in rect 1 {}", rect1.can_hold(&rect2));
    println!("rect 3 fits in rect 1 {}", rect1.can_hold(&rect3));
}
