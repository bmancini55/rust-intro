fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);

    let first = first_word(&s);

    println!("{}", first);
}

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
    return &s[..]
}