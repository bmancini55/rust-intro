pub fn run() {
    // creates an array with f_64 values
    let arr = [1_f64, 2_f64, 3_f64, 4_f64];

    // create the sum using the iterator function. This can also be written by
    // using the templated versino of sum instead of the implicit version:
    //      let sum = arr.iter().sum::<f64>();
    //      let sum: f64 = arr.iter().sum();
    let sum: f64 = arr.iter().sum();

    // creates a simple value
    let val: f64 = 100.1;

    // rounds a value and cassts to new type
    let total = (val.round() + sum) as i32;

    // finally assert the value is what is expected
    assert_eq!(total, 110);
}
