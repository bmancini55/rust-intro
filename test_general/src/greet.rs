pub fn run() {
    println!("{}", greet1());
    println!("{}", greet2());
    println!("{}", greet3());
}

fn greet1() -> String {
    "hello world!".to_string()
}

fn greet2() -> &'static str {
    "hello world!"
}

fn greet3() -> String {
    String::from("hello world!")
}
