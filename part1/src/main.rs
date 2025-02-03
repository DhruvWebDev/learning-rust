mod utils;
mod memory;
mod ownership;
mod borrwing_referencing;
mod r#struct;
mod r#enums;
mod generics_rust;
mod error_handling;
mod package;
fn main() {
    let a: i32 = -5;
    let y: u32 = 1000;
    let m: f32 = 1000.12345;

    println!("a:{}, y:{}, m:{}", a, y, m);

    let mut _x: u128 = 10;
    println!("Hello, world!");

    // for loop runs 1000 times now
    for i in 0..10 {
        _x = _x + 100;  // Explicit type conversion to match u128
        println!("_iteration:{}", i);
        println!("_x:{}", _x);
    }

    //Booleans 
    let is_male: bool = true;
    // is_male = false; it wont work and give compile error because the variable are immutable
    // To make the variable mutable we need to add a prefix mut...

    //if else 
    if is_male{
        println!("you are a male ");
    }else {
        println!("you are not a male");
    }

    //string 
    let greeting: String = String::from("Hello! World");
    println!("gretting: {}", greeting);

    // Get the 1st character from the string...
    let character: Option<char> = greeting.chars().nth(0);

    match character {
        Some(c) => println!("{}", c),  // Fix pattern matching syntax
        None => println!("no character at 0th index"),
    }
    let greet = greet(String::from("Dhruv Khandelwal"));
    println!("{}", greet);

    let first_word = get_first_word(String::from("Hello World"));
    println!("{}",first_word);

    //Print the message
    utils::print(String::from("Yoo guysss!!!!"));

    memory::memory_management();

    ownership::ownership_string();

    borrwing_referencing::borrowing_var();


    borrwing_referencing::mut_ref();

    r#struct::struct_use();
    r#struct::impl_struct();
    r#enums::enums_use();
    r#enums::edit_shape_enums_value();
    r#enums::get_first_a();
    generics_rust::generics_type();
    generics_rust::generics_enums();
    generics_rust::generics_tuple_struct();

    error_handling::error_handling();

    package::generate_random_number();
}


fn greet(name: String) -> String {
    let  greeting:String = String::from("Hello "); 
    
    greeting+ &name  // Concatenation is fine

}

fn get_first_word(sentence: String) -> String {
    let mut ans: String = String::from("");
    
    for char in sentence.chars() {  // Fixed `.char()` → `.chars()`
        ans.push(char);  // Fixed `.toString().as_str()` → `.push(char)`
        if char == ' ' {
            break;
        }
    }
    
    ans  // Implicit return
         // Rust, the last expression in a function is implicitly returned if there's no explicit return statement.
}



