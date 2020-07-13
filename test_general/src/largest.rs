pub fn run() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let string_list = vec!["hello", "world"];
    let result = largest_ref(&string_list);
    println!("The largest string is {}", result);

    let result = largest_cloned(&number_list);
    println!("The largest number is {}", result);

    let result = largest_cloned(&string_list);
    println!("The largest string is {}", result);
}

/// Implements via the Copy trait which restricts things to only
/// non-heap allocated types such as integers.
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/// We can also use a Clone type which will allocation the item on the
/// heap and return the owned value.
fn largest_cloned<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();

    for item in list {
        let item = item.clone();
        if item > largest {
            largest = item;
        }
    }

    largest
}

/// This version returns a reference to the value in the slice. This
/// prevents us from having to copy or perform any heap alloocations and
/// allows any type, not just those limited to copy! This is the most
/// efficient way to go about it
fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    &largest
}
