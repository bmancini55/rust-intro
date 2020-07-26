use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    let mut m = Memo::new(|num: i32| {
        println!("calc");
        num * 2
    });
    println!("{}", m.value(1));
    println!("{}", m.value(1));
    println!("{}", m.value(2));
    println!("{}", m.value(2));
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Memo::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

struct Memo<T, U>
where
    T: Fn(U) -> U,
    U: Clone + Copy + Eq + Hash,
{
    func: T,
    values: HashMap<U, U>,
}

impl<T, U> Memo<T, U>
where
    T: Fn(U) -> U,
    U: Clone + Copy + Eq + Hash,
{
    fn new(func: T) -> Memo<T, U> {
        Memo {
            func,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> U {
        if self.values.contains_key(&arg) {
            return self.values.get(&arg).unwrap().to_owned();
        } else {
            let val = (self.func)(arg);
            self.values.insert(arg, val);
            return val;
        }
    }
}
