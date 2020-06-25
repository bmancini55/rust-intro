pub fn run() {
    let m1 = Message::Write(String::from("hello"));
    m1.call();

    let m1 = Message::Quit;
    m1.call();
}

enum Message {
    Quit,
    // Move { x: i32, y: i32 },
    Write(String),
    // ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("called enum");
    }
}
