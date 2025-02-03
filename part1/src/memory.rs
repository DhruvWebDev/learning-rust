pub fn memory_management(){
    stack_memory();
    heap_memory();
    update_string();
}

fn stack_memory() {
    let a:i8 = 10;
    let b:i8 = 12;
    let c= a+b;
    println!("a:{} b:{} a+b:{}", a,b,c);
}

fn heap_memory() {
    let s:String = String::from("Hello ");
    let s1:String = String::from("World");
    println!("s:{} s1:{}", s, s1);
    let combined_string = format!("{} {}", s,s1);
    println!("Heap function : Combined string is{}", combined_string);
}

fn update_string(){
    let mut s:String = String::from("Intial String");
        println!("Before updating the string {}", s);
 println!("Before updating the string, the capacity:{}, length:{}, pointer:{:p}", s.capacity(), s.len(), s.as_ptr());
    s.push_str("some more test or data");
    println!("After updating the string {}", s);
    println!("After updating the string, the capacity:{}, length:{}, pointer:{:p}", s.capacity(), s.len(), s.as_ptr());

}
