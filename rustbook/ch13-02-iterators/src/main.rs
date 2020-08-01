fn main() {
    let v1 = vec![1, 2, 3];

    // Constructs an iterator
    let v1_iter = v1.iter();

    // Using a for loop takes ownership of the iterator and consumes it
    for val in v1_iter {
        println!("{}", val);
    }

    // Using an iterator adatper (map) and collecting the results into
    // a new collection type
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
