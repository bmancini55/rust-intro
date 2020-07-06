fn main() {
    // creates an empty string. This is a mutable, owned, UTF-8 encoded string
    let s = String::new();
    println!("empty String: {}", s);

    // Convert a string literal to a String by using the to_string() method.
    // This method is available on any type that implements the Display trait,
    // as string literals do
    let data = "some contents";
    let s = data.to_string();
    println!("String from string literal: {}", s);

    // Construct a string using String::from. This is the equivalent to the
    // to_string method for a string literal. But that's ok.
    let s = String::from("initial contents");
    println!("String::from: {}", s);

    // Can also encode some UTF-8 data
    let s = String::from("こんにちは");
    println!("UTF8 data: {}", s);

    // Can push strings onto an existing string. The push_str method does not
    // take ownership of the supplied string literal.
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    println!("push_str: {}, second {}", s, s2);

    // We can combine two values. S1 will no longer be valid once s3 is created
    // but s2 will still exist since it is a reference. We can only add a &str
    // to a String. We cannot add two String values together.
    let s1 = String::from("hello ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    println!("combined strings: {}", s3);

    // If we are performing a bunch of concat operations, we can use the format!
    // macro instead.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // Unlike other languages, we cannot access the character by numeric index
    // eg: hello[0]. Under the covers, a string is a Vec[u8]. We can convert to
    // a byte slice.
    let hello = "Здравствуйте";
    println!("{:?}", hello.as_bytes());

    // You can also use a string slice to get individual characters. However,
    // if we have an invalid length (eg: cross a character byte) we will panic.
    let s = &hello[0..4];
    println!("string slice: {}", s);

    // Instead we can use the .get(..) method to return the invividual characters
    // encompassing the bytes for the supplied range. You would need to be aware
    // the the utf8 characters are indeed 2-byte values. Invalid ranges will
    // return None
    let s = hello.get(0..4).unwrap_or("invalid range");
    println!("char 0: {}", s);

    // We can iterate the characters in a string using the .chars() method.
    for c in hello.chars() {
        println!("char {}", c);
    }

    // However, you should be aware that this is the individual Unicode scalar
    // values. So even though we have 4 "characters" in our grapheme cluster,
    // it has six values of char that we have here.
    for c in "नमस्ते".chars() {
        println!("char {}", c);
    }

    // We can also use the .bytes() method to iterate over the raw bytes
    for b in "नमस्ते".bytes() {
        println!("byte {}", b);
    }

    // How do you get the grapheme clusters from strings? Well that's not in the
    // standard library but there are crates for it.
}
