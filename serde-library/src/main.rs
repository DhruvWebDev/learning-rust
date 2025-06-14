use serde::{Serialize, Deserialize};
use serde_json::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE", deny_unknown_fields)]
pub struct Book {
    name: String,
    code: u128,
    description: Description,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE", deny_unknown_fields)]
pub struct Description {
    pages: i64,
    summary: String,
}

fn main() {
    let atomic_habit_book = Book {name:String::from("Atomic Habit"), code: 102, description:{ Description {pages: 100, summary: "it is a self help book.".to_string()}}};
    println!("{:#?}", atomic_habit_book);
    println!("Hello, world!");

    let json_ser = to_string_pretty(&atomic_habit_book);
    match json_ser {
    Ok(n) => println!("Got json stringified data type --> {}", n),
    error => println!("{:#?}", error),
}
    let stringified_json = r#"
    {
  "NAME": "Atomic Habit",
  "CODE": 102,
  "DESCRIPTION": {
    "PAGES": 100,
    "SUMMARY": "it is a self help book."
  },
  "HELLO":"dhruvvvv!!!"
}
"#;

let des = from_str::<Book>(stringified_json);
//println!("des  {}", des);

//`Result<Book, serde_json::Error>` cannot be formatted with the default formatter
match des {
    Ok(n) => println!("Got this struct {:#?}", n),
    error => println!("error!!!!!    {:#?}", error),

};

//to create a private enum or another ds

pub(crate) enum ErrorCode {
    NotFound,
    PermissionDenied,
    Timeout,
}

✅ You can use ErrorCode::NotFound inside the same crate

❌ Another crate that depends on this one cannot access ErrorCode


}
