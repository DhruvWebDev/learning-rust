fn reference() {
    let s1 = String::from("Hello");
    let s2 = &s1;

    println!("{}", s2);
    println!("{}", s1);// This is valid, The first pointer wasn't invalidated


}

pub fn borrowing_var(){
    let s1:String = String::from("Hello World");
    borrow_variable(&s1); //Passing a ref to the s1 variable 
    println!("{}", s1);
}

fn borrow_variable(variable: &String){
    println!("Referencing : {}", variable);
}

//Mutable Referencing
 
fn mut_borrow_var (s1: &mut String){
    s1.push_str("World!!!");
    println!("Mutable References : {}", s1);
}
pub fn mut_ref(){
    let mut s1:String = String::from("Hello ");
    mut_borrow_var(&mut s1)
}

//We can only have one mutable references at a time 
//But we can have multiple mutable referencesm at a time 
//If there is a mutable reference , you can’t have another immutable reference either.
