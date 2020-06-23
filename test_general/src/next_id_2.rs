pub fn run() {
    assert_eq!(next_id(&[0, 1, 2, 4, 5]), 3);
    assert_eq!(next_id(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 11);
    assert_eq!(next_id(&[1, 2, 0, 2, 3]), 4);
    println!("next_id_2 complete");
}

fn next_id(ids: &[usize]) -> usize {
    let mut sorted = ids.to_owned(); // could also use to_vec
    sorted.sort();
    if sorted.len() == 0 || sorted[0] != 0 {
        return 0;
    } else {
        let mut last: usize = 0;
        for &id in sorted.iter() {
            if id - last > 1 {
                return last + 1;
            }
            last = id;
        }
        last + 1
    }
}
