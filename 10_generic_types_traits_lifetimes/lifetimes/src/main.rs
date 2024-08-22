use std::fmt::Display;

// Because lifetimes are a type of generic, the declarations of the lifetime parameter 'a and the generic type parameter T go in the same list inside the angle brackets after the function name.
fn longest_with_an_announcement<'a, T>(x:&'a str, y: &'a str, ann: T) -> &'a str where T: Display {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// static lifetime, will live for duraiton of program
// let s: &'static str = "I have a static lifetime.";


fn main() {

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt { part: first_sentence };


    // this will not work because the reference to x is dropped before it is used.
    
    // let r;                // ---------+-- 'a
    //                       //          |
    // {                     //          |
    //     let x = 5;        // -+-- 'b  |
    //     r = &x;           //  |       |
    // }                     // -+       |
    //                       //          |
    // println!("r: {r}");   //          |

    // this will work. 

    // let x = 5;
    // let r = &x;

    // println!("r: {r}");



    // let string1 = String::from("abcd");
    // let string2 = "xyz";

    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {result}");

    let string1 = String::from("long string is long");
    let result: &str = "";

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("In scope: The longest string is '{result}'");
    }
    
    println!("Out Scope: The longest string is {result}");
}                         // ---------+
