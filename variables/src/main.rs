fn main() {

    // Mutability
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of inner scope x is {x}");
    }
    println!("The value of x is {x}");

    // u32
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");

    // Bools
    let t = true;
    let f: bool = false;
    println!("{t}{f}");

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup; // destructuring
    println!("The value of y is: {y}");
    let b = tup.0;
    println!("The value of tup.0 is: {b}");
    // arrays
    let _arr: [i32; 5] = [1,2,3,4,5];

    // functions
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}