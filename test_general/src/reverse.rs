pub fn run() {
    println!("reversed: {}", reverse("hello"));
    println!("reversed: {}", reverse2("hello"));
}

fn reverse(word: &str) -> String {
    word.chars() // creates an iterator [char] called Char
        .rev() // reverse directiono of the iterator
        .collect() // collect turns a Char into String, because String is
                   // just a collection of chars
}

fn reverse2(word: &str) -> String {
    let mut result = String::new();
    for c in word.chars().rev() {
        result.push(c);
    }
    result
}
