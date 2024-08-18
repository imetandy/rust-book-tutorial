#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

let config_max = Some(3u8);

// match example
match config_max {
    Some(max) => println!("The maximum is configured to be {max}"),
    _ => (),
}

// if let example
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}

let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}

if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}

fn main() {
    println!("Hello, world!");
}
