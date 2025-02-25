use std::thread;
fn main() {
    println!("Hello, world!");
    let handle = thread::spawn(|| {
        for i in 0..234 {
            println!("Hello from {} from spawn thread", i);

        }
    });

    let v = vec![1,2,3];

 let handle1 = thread::spawn(move || {
        for i in 0..234 {
            println!(" vector from spawn thread{:?}", v);
            println!("Hello WORLD FORM {}  from spawn thread", i);
        }
    });
    // It wont work because the vector v is moved to the closure and the main thread cannot access it
    // println!("{:?}", v);
    // handle.join().unwrap():
    // Waits for a thread to finish (join()).
    // Panics if the thread panicked (unwrap()).
    // Useful for managing concurrent execution and error handling in Rust.
    // Guarantees that all code in the thread completes before continuing.
    for i in 0..300{
        println!("hello from {} from main thread", i);
    };
    
}
//Since our new thread runs in parallel, the stack frame containing x and y may well have
//disappeared by the time we try to use them. Even if we call thr.join() within foo (which
//blocks until thr has completed, ensuring the stack frame won't disappear), we will not
//succeed: the compiler cannot prove that this behavior is safe, and so won't let us do it.

//The solution to this problem is usually to switch to using a move closure. This approach
//moves (or copies, where possible) data into the closure, rather than taking references to
//it. For example:

//fn foo() -> Box<Fn(u32) -> u32> {
//    let x = 0u32;
//    Box::new(move |y| x + y)
//}


// // main.rs or lib.rs
// pub fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = result(2, 2);
//         assert_println!("{:?}", v)handle1.join().unwrap();
