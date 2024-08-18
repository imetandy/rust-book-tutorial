fn main() {
    let mut s = String::new();
    let data = "Initial Contents";

    let mut s = data.to_string();
    // works in same way as above
    let mut s = "Initial Contents".to_string();

    // and also this
    let mut s = String::from("initial contents");

    s.push_str(" and more contents");

    println!("{:?}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let string1 = String::from("Hello, ");
    let string2 = String::from("world!");
    let string3 = string1 + &string2; // note string1 has been moved here. Can't use it again 


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // this is unweildy
    // let st = s1 + "-" + &s2 + "-" + &s3;
    // this is better
    let st = format!("{s1}-{s2}-{s3}");
    println!("{:?}", st);

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{:?}", s);

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}
