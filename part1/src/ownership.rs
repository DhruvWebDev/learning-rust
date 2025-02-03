pub fn ownership() {
    let s1:String = String::from("Hello World!!!");
    let s2:String = s1;
    //Owner of the data hello world is now the s2 and the s1 has moved
    println!("{}", s2);


    //Ownership work differently for string...

}

//Instead of cloning the s1 in the pass ownership fn as it is more expensive function instead i
//passed and returned the control to the s1
pub fn ownership_string() {
    let mut s1: String = String::from("Hello World");
    s1 = pass_ownership(s1);  // Ownership is moved and returned
    println!("s1: {}", s1);   // ✅ No error, s1 gets ownership back
}

fn pass_ownership(sentencee: String) -> String {  // Specify return type
    println!("sentencee: {}", sentencee);
    sentencee  // Return ownership
}

