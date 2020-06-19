fn main() {
    // string slices
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
    let first = first_word(&s);
    println!("{}", first);

    // calculate the width of a rectangle struct
    let rect1 = Rectangle { height: 96, width: 54 };
    println!("rect1 is {:?}", rect1);
    println!(
        "The area of the rectangle is {} square pixeld",
        area(&rect1)
    );

    // use a method to calculate it    
    println!(
        "The area of the rectable with the method is {} square pixels",
        rect1.area()
    );

    let rect2 = Rectangle { height: 100, width: 50 };
    let rect3 = Rectangle { height: 72, width: 12 };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(4);
    println!("Square {:?}", square);
}

// string slices
fn change(some_string: &mut String) {
    some_string.push_str(", world")
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){ 
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}


// struct stuff
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}


fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

// methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { height: size, width: size }
    }
}
