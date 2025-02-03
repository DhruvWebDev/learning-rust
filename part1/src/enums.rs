#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

pub fn enums_use() {
    println!("The enum north:{:?}, east:{:?} and so on...", Direction::North, Direction::East);
}

//In rust, enums can have associated value with them  --> this make the enums in rust much more
//powerful

enum Shape {
    Circle(f64),        
    Square(f64),        
    Rectangle(f64, f64), 
    None,  // Represents no shape
}

// Function to calculate area based on the shape
fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(side) => side * side,
        Shape::Rectangle(width, height) => width * height,
        Shape::None => 0.0,  // No shape means area is zero
    }
}

pub fn edit_shape_enums_value() {
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);
    let no_shape = Shape::None;
    
    println!("Circle Area: {}", calculate_area(circle));
    println!("Square Area: {}", calculate_area(square));
    println!("Rectangle Area: {}", calculate_area(rectangle));
    println!("No Shape Area: {}", calculate_area(no_shape));
}


// Enum Type 
//Rust doesnt have a null
//Find first a


fn first_a(word: String) -> Option<i32> {
    for (index, character) in word.chars().enumerate() {
        match character {
            'a' => {
                return Some(index as i32); // Return the index when 'a' is found
            },
            _ => {} // Do nothing for other characters
        }
    }
    None // Return None if 'a' is not found
}

pub fn get_first_a() {
    let my_string = String::from("raman");
    let returned_value = first_a(my_string);

    match returned_value {
        Some(index) => println!("The letter 'a' is found at index: {}", index),
        None => println!("The letter 'a' is not found in the string"),
    }
}

