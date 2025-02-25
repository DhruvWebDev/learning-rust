use std::collections::HashMap;
use std::io;
use std::fmt::Display;

trait Summary {
    fn summarize(&self) -> String;
}

struct User {
    name:String,
    age:u8,
}

impl Summary for User {
    fn summarize(&self) -> String{
        return format!("User {} is {} year old", self.name, self.age);
    }
} 

fn main() {
    println!("Give values for a vector (space-separated integers):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("input:{}", input);
    // Parse input string into a vector of integers
    let vector: Vec<i32> = input
        .trim()
        .split(",")
        .filter_map(|s| s.parse().ok())
        .collect();

    let even_vec = get_even_num(&vector);
    println!("vector:{:?}", vector);
    println!("Even numbers: {:?}", even_vec);

    //Initializing the vector using macros...
    let numbers:Vec<i32> = vec![1,2,3];
    for number in numbers {
        println!("numbers:{}", number);
    }

    //HashMaps --> Store key value pair
    let mut users:HashMap<String, i32> = HashMap::new();
    users.insert("Dhruv".to_string(), 14);
    let dhruv  = users.get("Dhruv");
    println!("{}", dhruv.unwrap());

    let input_vec = vec![(String::from("Dhruv"), 12),(String::from("Aarav"), 13)];

    let hm = group_value_by_key(&input_vec);
    println!("input vec: {:?}", input_vec);
    println!("hm:{:?}", hm);

    //Iterators
    //Here we are using for loop to iterate 
   let v1: Vec<i32> = vec![1, 2, 3];

    // Borrowing elements in the first loop
    for v in &v1 {
        println!("v1:{}", v);
    }

    // Explicitly define the iterator and then use for loop to iterate over Iterators
    let v1_iterator = v1.iter();  // Creates an iterator that borrows `v1`
    for v in v1_iterator {
        println!("v:{}", v);  // `v` is already a reference, no need to dereference
    }
    //Into Iteratorr
    //While let Some(res) = ...
    //Consumer Adapter --> it consumes the main interator
    //Iterator Adapter --> it just borrows the main iterator 
    
   let nums = vec![1, 2, 3, 4];

    let squared_iter = nums.iter().map(|&x| x * x); // Iterator adapter (not consumed yet)

    let sum: i32 = squared_iter.sum(); // ✅ Consumes the iterator

    //let sum_again: i32 = squared_iter.sum(); // ❌ ERROR: Iterator already consumed!

    println!("Sum: {}", sum);
    //println!("Sum again: {}", sum_again); // Won't work
   
    let vector_arr: Vec<i32> = vec![1, 2, 2];
    let total: i32 = vector_arr.iter().sum(); // OR vector_arr.into_iter().sum()
    println!("total: {}", total);

    //Strings v/s Slices  in rust  
    //Slices in rust let you reference a contiguous sequence of elements in a collections i.e.

    let mut word = String::from("Hello World");
    let word2 = &word[0..5];
    println!("word:{}", word2);

    let word:String = String::from("Helllo World"); 
    let first = first_word(&word);
    println!("first word = {}", first);

    //String Literal
    let str_lit:&str = "hello";
    println!("str_lit:{}",str_lit);

    let array = vec![1,2,3];
    let array_slice = &array[0..1];
    println!("array slice:{:?}", array_slice);


    //Generics --> Generics in Rust allow you to write flexible and reusable code that can work with multiple types without needing to specify exactly which type up front. You define a generic type placeholder, and it can later be replaced with any specific type when the code is used.
    //
    print_data::<i32>(42);          // Explicitly specifying the type `i32`
    print_data::<String>(String::from("Hello!")); // Explicitly specifying the type `String`

    let user = User {
        name : String::from("Dhruv"),
        age : 14
    };
    println!("{}", user.summarize());

    //Using impl Trait in Function Parameters
    print_value(1);
//The impl Trait syntax provides a concise way to indicate that a function parameter implements a specific trait. For example, consider a function that accepts any type implementing the Display trait:

    //Lifetimes 
    let ans;
    let str1:String = String::from("Hello");

    //Scope
    {
        let str2:String = String::from("DHRUV KHANDELWAL");
        ans  = longest(&str1, &str2);

          println!("The answer, the longest string is  :{}", ans);
    }

  
}

fn longest<'a> (a: &'a str, b:&'a str) -> &'a str{
    if a > b{
        return a;
    }else {
        return b;
    }
}
//Here in the Vec<i32> we have given the type of vector i.e. it is string , integer ...
fn get_even_num(vector: &Vec<i32>) -> Vec<i32> {
    let mut vec = Vec::new();
    for value in vector {
        match value % 2 {
            0 => vec.push(*value), // Even numbers are added to `vec`
            _ => (), // Do nothing for odd numbers
        }
    }
    vec
}


fn group_value_by_key(vec: &Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm = HashMap::new();
    for (key, value) in vec {
        hm.insert(key.clone(),*value); // key.clone() for ownership of String, *value for dereferencing i32
    }
    hm
}

fn first_word(word:&String) -> &str {
    let mut space_index = 0;
    for i in word.chars(){
        if i == ' '{
            break;
        }
        space_index = space_index + 1;
    }
    &word[0..space_index]
}


fn print_data<T:std::fmt::Display>(data:T){
    println!("{}", data);
}


fn print_value(value: impl Display) {
    println!("{}", value);
}
//In this example, the print_value function can accept any type that implements the Display trait, such as integers, strings, or custom types with a Display implementation.


