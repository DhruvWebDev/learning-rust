
#[derive(Debug)]
struct Point2D<A, B> {  // Renamed from Point to Point2D
    x: A,
    y: B,
}

pub fn generics_type() {
    let integer_point: Point2D<i32, i32> = Point2D {  // Use Point2D here
        x: 13,
        y: -14,
    };
    println!("integer_point -> x:{}, y:{}", integer_point.x, integer_point.y);
}

// Enum Color with a struct-like representation
#[derive(Debug)]
enum Color {
    RGB { red: i32, green: i32, blue: i32 },
}

pub fn generics_enums() {
    let black_color = Color::RGB {
        red: 0,
        green: 0,
        blue: 0,
    };

    match black_color {
        Color::RGB { red, green, blue } => {
            println!("Red component: {}", red);
            println!("Green component: {}", green);
            println!("Blue component: {}", blue);
        }
    }
}

// Renamed tuple struct to TuplePoint to avoid conflict
struct TuplePoint(i32, i32, i32);  // Renamed from Point to TuplePoint

pub fn generics_tuple_struct() {
    let p = TuplePoint(3, 5, 7);
    println!("tuple struct x: {}, y: {}, z: {}", p.0, p.1, p.2);  // Access tuple struct fields
}

fn main() {
    generics_type();
    generics_enums();
    generics_tuple_struct();
}

