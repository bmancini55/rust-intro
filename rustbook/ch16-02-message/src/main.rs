use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Demonstrates a simple example with a single message that is passed
    // to the main thread. The ownership of the value sent is transfered
    // to the receiver.
    {
        println!("\nEXAMPLE 1\n");
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }

    // Demonstrates multiple sends with a slow producer. The consumer
    // simply iterates over the values that it receives.
    {
        println!("\nEXAMPLE 2\n");
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_millis(200));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }

    // This is an example of a multiple producer channel. We clone the
    // sender and can use it in multiple thread! We perform a clone
    // before we use it by moving it into its own thread.
    {
        println!("\nEXAMPLE 3\n");
        let (tx, rx) = mpsc::channel();

        let tx1 = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_millis(200));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_millis(200));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }
}
