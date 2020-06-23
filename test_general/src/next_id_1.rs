pub fn run() {
    assert_eq!(next_id(&[0, 1, 2, 4, 5]), 3);
    assert_eq!(next_id(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 11);
    assert_eq!(next_id(&[1, 2, 0, 2, 3]), 4);
    println!("next_id_1 complete");
}

fn next_id(ids: &[usize]) -> usize {
    let mut id = 0;
    while ids.contains(&id) {
        id += 1;
    }
    id
}
