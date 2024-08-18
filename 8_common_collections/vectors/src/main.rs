// Vec<T>
// Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory. 


fn main() {

    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:#?}", v);


    let x = vec![1, 2, 3, 4, 5];
    let third: &i32 = &x[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = x.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }


// using enums to create a single type that can be held within a vector

enum Spredsheetcell {
    Int(i32),
    Float(f64),
    Text(String),
}
// now we can have a vector that holds 3 different types of data, but all are of the same type (spreadsheetcell)
let row = vec![
    Spredsheetcell::Int(3),
    Spredsheetcell::Text(String::from("blue")),
    Spredsheetcell::Float(10.12),
];

}
