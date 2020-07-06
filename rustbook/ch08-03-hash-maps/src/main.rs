use std::collections::HashMap;

fn main() {
    // example of creating a simple HashMap by creating an empty HashMap then
    // adding two items to it using the insert method
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    // we can use two different vectors of equal length along with the zip method
    // to construct a HashMap from and key and value vectors. Collect requires
    // specifying that we want a HashMap since we can collect into a variety of
    // different data types
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores);

    // Ownership is moved into the hashmap. For values that implement copy, the
    // value will be copied into the hashmap.
    let field_name = String::from("color");
    let field_value = String::from("blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // field_name and field_value are invalid at this point. However we can
    // instead pass in references, but the references must live as long as the
    // HashMap
    let field_name = String::from("color");
    let field_value = String::from("blue");
    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    println!("{}, {}, {:?}", field_name, field_value, map);

    // for example, the following would be incorrect:
    // let mut map = HashMap::new();
    // {
    //     let field_name = String::from("color");
    //     let field_value = String::from("blue");
    //     map.insert(&field_name, &field_value);
    // }

    // We can get the value of the HashMap by using it's key which will return
    // an Option(&i32) value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score);

    // We can also iterate the HashMap with a Key Value tuple. Each of these
    // will be a referenced value &String and &i32
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // We can overwrite an existing value by simply reinserting the new value
    // using the insert method we've already used
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 50);
    println!("{:?}", scores);

    // We can insert if there is no value by using the `entry` method and if
    // doesn't exist we can insert it
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // We can also update an existing value by mutating the existing value.
    // or_insert will return a mutable reference to the value which we can read
    // and modify
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
