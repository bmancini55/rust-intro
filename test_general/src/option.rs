pub fn run() {
    // Option::Some and Option::None are included in the std::prelude, so they
    // are available in scope without any imports.

    // Some<T> is templated, so you can pass in any type.
    let some_number = Some(5);
    let some_string = Some("hello");
    assert_eq!(some_number.unwrap(), 5);
    assert_eq!(some_string.unwrap(), "hello");

    // There are a bunch of methods that can convert an Option<T> type back to
    // its non-nullable type. `unwrap` is not recommended because it can throw.
    // https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.unwrap
    let sum = 5 + some_number.unwrap();
    assert_eq!(sum, 10);

    // When you use None, it will not have an implicit template type. So you
    // must specify the type.
    let absent_number: Option<i32> = None;

    // You can use unwrap_or("val")
    assert_eq!(absent_number.unwrap_or(5), 5);

    // Or you can use a delegate
    const MULTIPLIER: i32 = 2;
    assert_eq!(absent_number.unwrap_or_else(|| MULTIPLIER * 5), 10);

    // Or you can use the default for the type
    assert_eq!(absent_number.unwrap_or_default(), 0);
}
