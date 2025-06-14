struct Truck {
    next_truck: Option<Box<Truck>>,
}


fn main() {
    println!("Hello, world!");
        let mut truck1 = Truck { next_truck: None };
            let truck2 = Truck { next_truck: None };
        truck1.next_truck = Some(Box::new(truck2));

        helper();
        arc()
}

use std::rc::Rc;

fn helper() {
    //Reference count of an Rc increases by 1 whenever an Rc is cloned, and decreases by 1 whenever one cloned Rc is dropped out of the scope. When an Rc's reference count becomes zero (which means there are no remaining owners), both the Rcand the value are all dropped.
    let rc_examples = "Rc examples".to_string();
    {
        println!("--- rc_a is created ---");
        
        let rc_a: Rc<String> = Rc::new(rc_examples);                                         // strong_count = 1 because it is referencing the rc_example
                                                                                            
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
        
        {
            println!("--- rc_a is cloned to rc_b ---");
            
            let rc_b: Rc<String> = Rc::clone(&rc_a);
            println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));             //here
                                                                                          //strong_count
            
            // Two `Rc`s are equal if their inner values are equal
            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));
            
            // We can use methods of a value directly
            println!("Length of the value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}", rc_b); //here the the rc_b drops and doesnt point to the
                                                 //rc_a
            
            println!("--- rc_b is dropped out of scope ---");
        }
        
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));   //hence rc_a
                                                                            //strong_count is 1
        
        println!("--- rc_a is dropped out of scope ---");
    }
    
    // Error! `rc_examples` already moved into `rc_a`
    // And when `rc_a` is dropped, `rc_examples` is dropped together
    // println!("rc_examples: {}", rc_examples);
    // TODO ^ Try uncommenting this line
}


use std::time::Duration;
use std::sync::Arc;
use std::thread;

fn arc() {
    // This variable declaration is where its value is specified.
    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        // Here there is no value specification as it is a pointer to a
        // reference in the memory heap.
        let apple = Arc::clone(&apple);

        thread::spawn(move || {
            // As Arc was used, threads can be spawned using the value allocated
            // in the Arc variable pointer's location.
            println!("{:?}", apple);
        });
    }
    //make the main thread to sleep for 1 seconds Pauses the main thread for 1 second, giving all the spawned threads time to finish their work (printing the string).
    // Make sure all Arc instances are printed from spawned threads.
    // we pause the main thread till all the secondary threads run and finish because when the main
    // threads exits, the program ends...
    thread::sleep(Duration::from_secs(1));
}
