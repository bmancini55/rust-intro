pub fn run() {
    // create an empty vector
    let v: Vec<i32> = Vec::new();
    println!("vec empty {:?}", v);

    // create a vector with stuff in it using the vec macro
    let v = vec![1, 2, 3];
    println!("vec with values {:?}", v);

    // creating a vector and dynamically pushing values into it
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // accessing a value by index
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("third value is {}", third);

    // accessing a value with get, which returns an optional value, this allows
    // us to try to get values that may be outside the scope of the vec without
    // causing a panic
    match v.get(2) {
        Some(third) => println!("third value is {}", third),
        None => println!("there is no third value"),
    }

    // reads the values in the vector by iterating over them
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // iterate over mutable references and modify the values in the vector
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }
}
