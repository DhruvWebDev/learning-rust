struct User {
        name: String, 
        age: u32,
        active: bool
}

struct Color(i32,i32,i32);
pub fn struct_use() {
    let user:User = User {
       name:String::from("Hello World"),
       age:10,
       active:true
    };

    println!("name:{}, age:{}, active:{}", user.name, user.age, user.active);


    //Tuple struct...
    let black = Color(0,0,0);
    println!("Red:{}, green:{}, blue:{}", black.0, black.1, black.2);
}


struct Rect {
   width: u32,
   height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
         self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2* (self.height + self.width)
    }
}

pub fn impl_struct() {
    let rect = Rect {
        width: 30,
        height: 50,
    };
    print!("The area of the rectangle is {}", rect.area());
    println!("the perimeter of the rectangle id {}", rect.perimeter());
}
