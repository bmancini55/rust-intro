use crate::List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list: {:?}", list);

    // Creates an actual linked list
    let mut list = LinkedList::new(1);
    LinkedList::add(&mut list, 2);
    LinkedList::add(&mut list, 3);
    println!("list: {:?}", list);
    println!("last: {:?}", LinkedList::find_end(&list))
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
struct LinkedList<T> {
    value: T,
    next: Option<Box<LinkedList<T>>>,
}

impl<T> LinkedList<T> {
    fn new(val: T) -> Box<LinkedList<T>> {
        Box::new(LinkedList {
            value: val,
            next: None,
        })
    }
    fn add(list: &mut Box<LinkedList<T>>, val: T) {
        let last = LinkedList::find_end_mut(list);
        last.next = Some(Box::new(LinkedList {
            value: val,
            next: None,
        }))
    }
    fn find_end(list: &Box<LinkedList<T>>) -> &Box<LinkedList<T>> {
        let mut last = list;
        while last.next.is_some() {
            last = last.next.as_ref().unwrap();
        }
        last
    }

    fn find_end_mut(list: &mut Box<LinkedList<T>>) -> &mut Box<LinkedList<T>> {
        let mut last = list;
        while last.next.is_some() {
            last = last.next.as_mut().unwrap();
        }
        last
    }
}

// Possible fun exercise...
// https://codereview.stackexchange.com/questions/204183/doubly-linked-list-in-rust
