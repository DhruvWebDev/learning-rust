use std::thread;
use std::sync::mpsc;
fn main() {
    // println!("Hello, world!");
    // let handle = thread::spawn(|| {
    //     for i in 0..234 {
    //         println!("Hello from {} from spawn thread", i);

    //     }
    // });

    // let v = vec![1,2,3];

//  let handle1 = thread::spawn(move || {
//         for i in 0..234 {
//             println!(" vector from spawn thread{:?}", v);
//             println!("Hello WORLD FORM {}  from spawn thread", i);
//         }
//     });
    // It wont work because the vector v is moved to the closure and the main thread cannot access it
    // println!("{:?}", v);
    // handle.join().unwrap():
    // Waits for a thread to finish (join()).
    // Panics if the thread panicked (unwrap()).
    // Useful for managing concurrent execution and error handling in Rust.
    // Guarantees that all code in the thread completes before continuing.
    // for i in 0..300{
    //     println!("hello from {} from main thread", i);
    // };

    //MESSAGE PASSING  --> CHANNEL
    //Channel
    //A channel is a way to send multiple values from one thread to another. One part of your code
    //Usecase--> reading data from redis in one thread and then passing it another thread...

    // let (tx, rx) = mpsc::channel();

    // let handle = thread::spawn(move || {
    //     for i in 0..10 {
    //         println!("Sending: {}", i);
    //         tx.send(i).unwrap();
    //     }
    // });
    // // /You can't print rx directly because it's a Receiver, not a value.
    // // Loop to receive all values
    // for received in rx {
    //     println!("Got: {}", received);
    // }

    // handle.join().unwrap();


    //In this problem, I made the computing 1..10^8 in 4 threads and then summing up the results in the main thread.
    //this made the load on the single thread to multiple thread and made the computation time less 
    let (tx, rx) = mpsc::channel();

    // Clone tx for each thread
    let tx1 = tx.clone();
    let tx2 = tx.clone();
    let tx3 = tx.clone();
    let tx4 = tx.clone();
    
    let handle_1 = thread::spawn(move || {
        let mut sum: i64 = 0;
        for i in 1..10_i64.pow(2) {
            sum += i;
        }
        tx1.send(sum).unwrap();
    });
    
    let handle_2 = thread::spawn(move || {
        let mut sum: i64 = 0;
        for i in 10_i64.pow(2)..10_i64.pow(4) {
            sum += i;
        }
        tx2.send(sum).unwrap();
    });
    
    let handle_3 = thread::spawn(move || {
        let mut sum: i64 = 0;
        for i in 10_i64.pow(4)..10_i64.pow(6) {
            sum += i;
        }
        tx3.send(sum).unwrap();
    });
    
    let handle_4 = thread::spawn(move || {
        let mut sum: i64 = 0;
        for i in 10_i64.pow(6)..10_i64.pow(8) {
            sum += i;
        }
        tx4.send(sum).unwrap();
    });
    
    // Drop the original tx to avoid deadlock
    drop(tx);
//     Messages are printed as they are received, not all at once.
// But the loop never ends, because the channel isn't closed.
// This is because Rust's channel checks if all senders are dropped to know it’s safe to exit.
    
    let mut final_sum: i64 = 0;
    for received in rx {
        final_sum += received;
        println!("Got: {}", received);
    }
    println!("Final Sum: {}", final_sum);
    //Threads don’t necessarily finish in order, but the main thread processes the messages in the order they are received.

    //In Rust, lifetimes are needed when you use references to ensure memory safety. They help the compiler understand how long references are valid, preventing dangling pointers and data races.

    // Use lifetimes to link the scope of references.
    // They ensure borrowed data lives long enough to be safely accessed.
    // Rust often infers lifetimes automatically, but you specify them when it's ambiguous.
    
//     Generic Functions and Traits
// When working with generic functions or implementing traits that involve references.
// You need to specify lifetimes to ensure the references are valid.
// For example, this function won't compile without a lifetime:
// fn print_ref<'a>(x: &'a i32) {
//     println!("{}", x);
// }
// The lifetime 'a specifies that the reference must live long enough for the function to use it.

}
