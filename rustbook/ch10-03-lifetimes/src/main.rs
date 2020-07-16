fn main() {
    // We can pass string slices which are references to a function and
    // have that function return another string slice, so long as we
    // supply the lifetime.
    let a = String::from("hello");
    let b = "world!";
    println!("longest: {}", longest(&a[..], b));

    // We can also have structs contain a reference, but the struct must
    // use lifetimes as well.
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("line {}", i.part);

    // A method of a struct will use the lifetime of the struct. So
    // there may not be a need for using a lifetime
    let p = i.announce_and_return_part(&"I have a message!");
    println!("part {}", p);
}

/// String slice needs to have a lifetime associated with it. Otherwise
/// it will not know the lifetime of the referenced value it is returning
/// with. We can resolve this by having the lifetime of each argument
/// established.
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

/// Struct with a reference requires us to have a
struct ImportantExcerpt<'a> {
    part: &'a str,
}

/// Methods that reference self however will pick up the lifetime of
/// self. Since self is thte lifetime of 'a, then the method does not
/// need lifetime tags. This falls under lifetime elision rules.
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
