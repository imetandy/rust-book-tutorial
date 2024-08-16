
// enum IpAddr {
//     V4(u8,u8,u8,u8),
//     V6(String),
// }
// let home = IpAddr::V4(127,0,0,1);
// let loopback = IpAddr::V6(String::from("::1"));

// enum IpAddrKind {
//     V4(String),
//     V6(String),
// }
// let home = IpAddrKind::V4(String::from("127.0.0.1"));
// let loopback = IpAddrKind::V6(String::from("::1"));

// if we use structs we have all this exrta code, instead of just passing string into the enum. 
    // struct IpAddr {
    // kind: IpAddrKind,
    // address: String,
    // }

    // let home = IpAddr {
    // kind: IpAddrKind::V4,
    // address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };


// Standard library also implements this.
    // struct Ipv4Addr {
    //     // --snip--
    // }
    
    // struct Ipv6Addr {
    //     // --snip--
    // }
    
    // enum IpAddr {
    //     V4(Ipv4Addr),
    //     V6(Ipv6Addr),
    // }


struct QuitMessage; // unit struct

struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct


impl Message {
    fn call (&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();

// OPTIONS

let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;




fn main() {
    println!("Hello, world!");
}
