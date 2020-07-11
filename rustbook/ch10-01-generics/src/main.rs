fn main() {
    // Example of a struct with a single generic type and defining generic
    // methods definitions for the struct.
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    // We can also define a specific type that allows us to attach methods
    // that can only be used when we are using that type
    let p = Point { x: 1.0, y: 1.0 };
    println!("disance_from_origin = {}", p.distance_from_origin());

    // We can also implement generics where the method has types other than
    // the types defined in our generic struct.
    let p = MultiPoint { x: 5, y: 10.0 };
    let p = p.mixup(MultiPoint {
        x: "hello",
        y: "world",
    });
    println!("mixed up x: {}, y: {}", p.x, p.y);
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct MultiPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> MultiPoint<T, U> {
    fn mixup<V, W>(self, other: MultiPoint<V, W>) -> MultiPoint<T, W> {
        MultiPoint {
            x: self.x,
            y: other.y,
        }
    }
}
