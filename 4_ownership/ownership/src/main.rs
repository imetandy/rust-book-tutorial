
fn main() { // s not valid here as its not declared yet
    let mut s = String::from("Hello"); // s is valid from this point forward

    s.push_str(", world!"); // push_str() appends a literal to a string
    println!("{s}");

    let s1 = String::from("hello");
    let s2 = s1; // this is a copy of the pointer, not of the data
    // s1 is now no longer valid. This is a move, not a copy
    println!("{s2}");




    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}"); // integers have known size at compile time, so they are stored on the stack and dont require cloning.


    // clone

    let c1 = String::from("hello");
    let c2 = c1.clone(); // this is a copy of the data on the heap. 

    println!("c1 = {c1}, c2 = {c2}");

    testing_ownership();

    return_multiple_vals();

} // scope over, s is no longer valid

fn testing_ownership() {
    
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);


    println!("s1 = {s1}, s3 = {s3}");
}

fn gives_ownership() -> String {
    let some_string = String::from("Yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn return_multiple_vals() {
let s1 = String::from("Hello");
let (s2, len) = calculate_length(s1);

println!("The length of '{s2}' is {len}");

}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)

}