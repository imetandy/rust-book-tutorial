use std::net:IpAddr;

let home: IpAddr = "127.0.0.1".parse().expect("Hardcoded IP address should be valid");

// custom validation type 
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value <1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess {value}
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}
loop {
    // -- snip --
    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    if guess < 1 || guess > 100 {
        println!("the secret number will be between 1 and 100");
        continue;
    }

    match guess.cmp(&secret_number){
        // -- snip -- 
    }
}




fn main() {
    println!("Hello, world!");
}
