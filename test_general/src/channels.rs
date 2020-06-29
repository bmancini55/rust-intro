use std::sync::mpsc::channel;
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::SyncSender;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

pub fn run() {
    receive_nums();
    receive_nums_sync();
    receive_nums_sync_with_result();
    receive_nums_sync_loop();
}

/// Non-blocking producer / consumer example. This example spawns a new thread
/// with a producer that pushes as much data into the channel as possible. The
/// consumer will slowly read the information.
fn receive_nums() {
    println!("non-blocking producer");
    // Creates a thread and unbuffered channel which will allow the producer to
    // push as much information as possible.
    let (sender, receiver) = channel();
    thread::spawn(move || stream_nums(&sender));

    // block to allow the producer to do its thing
    sleep(Duration::from_millis(100));

    // consume the numbers in a blocking manner from the iterator
    for num in receiver.iter() {
        println!("received: {}", num);
    }
}

/// Produces the numbers as fast as possible
fn stream_nums(sender: &Sender<i32>) {
    for i in 1..10 {
        sender.send(i).unwrap();
        println!("produced: {}", i);
    }
}

/// Uses a synchronous channel that will only allow a single value to pushed
/// into the buffer at a time. If more than one value is pushed, the producer
/// will block until the consumer has processed the information.
fn receive_nums_sync() {
    println!("\nsync channel with a slow receiver");
    // Creates a synchronous channel that blocks until the consumer has read the
    // information from the channel.
    let (sender, receiver) = sync_channel(1);
    thread::spawn(move || stream_nums_sync(&sender));

    // block to allow the producer to do its thing
    sleep(Duration::from_millis(100));

    // Uses an iterator to read the value synchronously from the channel
    for num in receiver.iter() {
        println!("received: {}", num);
        sleep(Duration::from_millis(100));
    }
}

/// Streams number synchronously which will block until the consumer has read
/// information
fn stream_nums_sync(sender: &SyncSender<i32>) {
    for i in 1..10 {
        sender.send(i).unwrap();
        println!("produced: {}", i);
    }
}

/// Uses a synchronous channel that will only allow a single value to be pushed
/// into the buffer at a time. If more than one value is pushed, the producer
/// will block until the consumer has proccessed the information
fn receive_nums_sync_loop() {
    println!("\nsync channel with a slow producer");
    let (sender, receiver) = sync_channel(1);
    thread::spawn(move || stream_nums_sync_slow(&sender));

    // block to allow the producer to do its thing
    sleep(Duration::from_millis(100));

    // Uses a loop to read information from the channel synchronously
    loop {
        match receiver.recv() {
            Ok(val) => println!("received: {}", val),
            Err(_) => break,
        }
    }
}

/// Streams number synchronously slowly
fn stream_nums_sync_slow(sender: &SyncSender<i32>) {
    for i in 1..10 {
        sender.send(i).unwrap();
        println!("produced: {}", i);
        sleep(Duration::from_millis(100));
    }
}

fn receive_nums_sync_with_result() {
    println!("\nsync channel with thread result");
    // Construct a sync_channel that will ensure that reads and writes occur in
    // a synchronous mannger by blocking the thread.
    let (sender, receiver) = sync_channel(1);
    let child = thread::spawn(move || stream_nums_sync_result(&sender));

    // block to allow the producer to do its thing
    sleep(Duration::from_millis(100));

    // receive the data on the main thread
    for num in receiver.iter() {
        println!("received: {}", num);
    }

    // finally after the producer thread has exited, we can view the result
    // to see if things were ok
    match child.join().unwrap() {
        Ok(v) => println!("returned value: {}", v),
        Err(e) => println!("returned error: {}", e),
    }
}

fn stream_nums_sync_result(sender: &SyncSender<i32>) -> Result<String, String> {
    for i in 1..10 {
        match sender.send(i) {
            Ok(_) => println!("produced: {}", i),
            Err(_) => break,
        }
    }
    Err(String::from("some error"))
    // Ok(String::from("all is good"))
}
